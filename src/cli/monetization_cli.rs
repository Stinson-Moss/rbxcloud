//! `game-pass` and `developer-product` commands.

use clap::{Args, Subcommand};
use rbxcloud::rbx::{
    types::UniverseId,
    v2::{developer_product::DeveloperProductFields, game_pass::GamePassFields, ImageUpload},
};

use super::to_json;

#[derive(Debug, Args)]
struct Common {
    /// Universe ID of the experience
    #[clap(short, long, value_parser)]
    universe_id: u64,

    /// Pretty-print the JSON response
    #[clap(short, long, value_parser, default_value_t = false)]
    pretty: bool,

    /// Roblox Open Cloud API Key
    #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
    api_key: String,
}

/// Fields shared by game passes and developer products.
#[derive(Debug, Args)]
struct ProductFields {
    /// Name
    #[clap(short, long, value_parser)]
    name: Option<String>,

    /// Description
    #[clap(short, long, value_parser)]
    description: Option<String>,

    /// Price in Robux
    #[clap(short = 'r', long, value_parser)]
    price: Option<u64>,

    /// Whether it can be purchased
    #[clap(short = 's', long, value_parser)]
    for_sale: Option<bool>,

    /// Icon image file (PNG, JPEG, BMP, or TGA)
    #[clap(long, value_parser)]
    icon: Option<String>,
}

impl ProductFields {
    fn image(&self) -> anyhow::Result<Option<ImageUpload>> {
        Ok(match &self.icon {
            Some(path) => Some(ImageUpload::from_path(path)?),
            None => None,
        })
    }
}

#[derive(Debug, Args)]
struct Paging {
    /// Maximum number of items to return
    #[clap(short, long, value_parser)]
    limit: Option<u32>,

    /// Page token from a previous response's `nextPageToken`
    #[clap(short, long, value_parser)]
    cursor: Option<String>,
}

// ------------------------------------------------------------ game passes

#[derive(Debug, Subcommand)]
enum GamePassCommands {
    /// List the universe's game passes
    List {
        #[clap(flatten)]
        paging: Paging,

        /// Only list passes that are for sale
        #[clap(long, value_parser, default_value_t = false)]
        for_sale_only: bool,

        #[clap(flatten)]
        common: Common,
    },

    /// Get a game pass
    Get {
        /// Game pass ID
        #[clap(short = 'g', long, value_parser)]
        game_pass_id: u64,

        #[clap(flatten)]
        common: Common,
    },

    /// Create a game pass (--name is required)
    Create {
        #[clap(flatten)]
        fields: ProductFields,

        #[clap(flatten)]
        common: Common,
    },

    /// Update a game pass; only the given fields change
    Update {
        /// Game pass ID
        #[clap(short = 'g', long, value_parser)]
        game_pass_id: u64,

        #[clap(flatten)]
        fields: ProductFields,

        #[clap(flatten)]
        common: Common,
    },
}

/// Manage a universe's game passes
#[derive(Debug, Args)]
pub struct GamePassCli {
    #[clap(subcommand)]
    command: GamePassCommands,
}

impl ProductFields {
    fn game_pass(&self) -> anyhow::Result<GamePassFields> {
        Ok(GamePassFields {
            name: self.name.clone(),
            description: self.description.clone(),
            price: self.price,
            is_for_sale: self.for_sale,
            image: self.image()?,
            ..Default::default()
        })
    }

    fn developer_product(&self) -> anyhow::Result<DeveloperProductFields> {
        Ok(DeveloperProductFields {
            name: self.name.clone(),
            description: self.description.clone(),
            price: self.price,
            is_for_sale: self.for_sale,
            image: self.image()?,
            ..Default::default()
        })
    }
}

impl GamePassCli {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            GamePassCommands::List {
                paging,
                for_sale_only,
                common,
            } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .game_passes(UniverseId(common.universe_id))
                    .list(paging.limit, paging.cursor, for_sale_only.then_some(true))
                    .await?;
                to_json(&res, common.pretty)
            }

            GamePassCommands::Get {
                game_pass_id,
                common,
            } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .game_passes(UniverseId(common.universe_id))
                    .get(game_pass_id)
                    .await?;
                to_json(&res, common.pretty)
            }

            GamePassCommands::Create { fields, common } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .game_passes(UniverseId(common.universe_id))
                    .create(&fields.game_pass()?)
                    .await?;
                to_json(&res, common.pretty)
            }

            GamePassCommands::Update {
                game_pass_id,
                fields,
                common,
            } => {
                let client = super::v2_client(&common.api_key)?;
                let passes = client.game_passes(UniverseId(common.universe_id));
                passes.update(game_pass_id, &fields.game_pass()?).await?;
                // The update endpoint returns no body; show the result.
                let res = passes.get(game_pass_id).await?;
                to_json(&res, common.pretty)
            }
        }
    }
}

// ----------------------------------------------------- developer products

#[derive(Debug, Subcommand)]
enum DeveloperProductCommands {
    /// List the universe's developer products
    List {
        #[clap(flatten)]
        paging: Paging,

        #[clap(flatten)]
        common: Common,
    },

    /// Get a developer product
    Get {
        /// Developer product ID
        #[clap(short = 'i', long, value_parser)]
        product_id: u64,

        #[clap(flatten)]
        common: Common,
    },

    /// Create a developer product (--name is required)
    Create {
        #[clap(flatten)]
        fields: ProductFields,

        #[clap(flatten)]
        common: Common,
    },

    /// Update a developer product; only the given fields change
    Update {
        /// Developer product ID
        #[clap(short = 'i', long, value_parser)]
        product_id: u64,

        #[clap(flatten)]
        fields: ProductFields,

        #[clap(flatten)]
        common: Common,
    },
}

/// Manage a universe's developer products
#[derive(Debug, Args)]
pub struct DeveloperProductCli {
    #[clap(subcommand)]
    command: DeveloperProductCommands,
}

impl DeveloperProductCli {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            DeveloperProductCommands::List { paging, common } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .developer_products(UniverseId(common.universe_id))
                    .list(paging.limit, paging.cursor)
                    .await?;
                to_json(&res, common.pretty)
            }

            DeveloperProductCommands::Get { product_id, common } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .developer_products(UniverseId(common.universe_id))
                    .get(product_id)
                    .await?;
                to_json(&res, common.pretty)
            }

            DeveloperProductCommands::Create { fields, common } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .developer_products(UniverseId(common.universe_id))
                    .create(&fields.developer_product()?)
                    .await?;
                to_json(&res, common.pretty)
            }

            DeveloperProductCommands::Update {
                product_id,
                fields,
                common,
            } => {
                let client = super::v2_client(&common.api_key)?;
                let products = client.developer_products(UniverseId(common.universe_id));
                products
                    .update(product_id, &fields.developer_product()?)
                    .await?;
                // The update endpoint returns no body; show the result.
                let res = products.get(product_id).await?;
                to_json(&res, common.pretty)
            }
        }
    }
}
