use clap::{Args, Subcommand};
use rbxcloud::rbx::v2::creator_store::{
    CreatorStoreAssetKind, CreatorStoreProduct, Money, SearchCategory, SearchParams, SearchSort,
};

use super::to_json;

#[derive(Debug, Args)]
struct Common {
    /// Pretty-print the JSON response
    #[clap(short, long, value_parser, default_value_t = false)]
    pretty: bool,

    /// Roblox Open Cloud API Key
    #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
    api_key: String,
}

/// Parse a USD price like `4.99` or `0` into cents.
fn parse_usd(price: &str) -> anyhow::Result<u64> {
    let (dollars, cents) = match price.split_once('.') {
        Some((d, c)) if c.len() <= 2 => (d, format!("{c:0<2}")),
        Some(_) => anyhow::bail!("price {price} has more than two decimal places"),
        None => (price, "00".to_string()),
    };
    let dollars: u64 = dollars
        .parse()
        .map_err(|_| anyhow::anyhow!("invalid price: {price}"))?;
    let cents: u64 = cents
        .parse()
        .map_err(|_| anyhow::anyhow!("invalid price: {price}"))?;
    Ok(dollars * 100 + cents)
}

#[derive(Debug, Subcommand)]
enum CreatorStoreCommands {
    /// Get a Creator Store product
    GetProduct {
        /// Product ID, e.g. from the `path` of a previous response
        #[clap(short = 'i', long, value_parser)]
        product_id: String,

        #[clap(flatten)]
        common: Common,
    },

    /// Distribute an asset on the Creator Store for the first time
    CreateProduct {
        /// Asset type
        #[clap(short, long, value_enum)]
        kind: CreatorStoreAssetKind,

        /// Asset ID
        #[clap(short = 'A', long, value_parser)]
        asset_id: u64,

        /// Price in USD, e.g. 4.99 (0 for free)
        #[clap(short = 'P', long, value_parser, default_value = "0")]
        price: String,

        /// Create the product without publishing it
        #[clap(long, value_parser, default_value_t = false)]
        unpublished: bool,

        #[clap(flatten)]
        common: Common,
    },

    /// Change a product's price or published state
    UpdateProduct {
        /// Product ID, e.g. from the `path` of a previous response
        #[clap(short = 'i', long, value_parser)]
        product_id: String,

        /// New price in USD, e.g. 4.99 (0 for free)
        #[clap(short = 'P', long, value_parser)]
        price: Option<String>,

        /// Publish (true) or unpublish (false) the product
        #[clap(short = 'u', long, value_parser)]
        published: Option<bool>,

        #[clap(flatten)]
        common: Common,
    },

    /// Search the Creator Store
    Search {
        /// Search text
        #[clap(short, long, value_parser)]
        query: Option<String>,

        /// Category to search
        #[clap(short, long, value_enum)]
        category: Option<SearchCategory>,

        /// Sort order
        #[clap(short, long, value_enum)]
        sort: Option<SearchSort>,

        /// Sort descending
        #[clap(short, long, value_parser, default_value_t = false)]
        descending: bool,

        /// Only show assets from this user
        #[clap(long, value_parser)]
        user_id: Option<u64>,

        /// Only show assets from this group
        #[clap(long, value_parser)]
        group_id: Option<u64>,

        /// Only show assets from verified creators
        #[clap(long, value_parser, default_value_t = false)]
        verified: bool,

        /// Maximum number of results
        #[clap(short, long, value_parser)]
        limit: Option<u32>,

        /// Page token from a previous response's `nextPageToken`
        #[clap(short = 't', long, value_parser)]
        page_token: Option<String>,

        #[clap(flatten)]
        common: Common,
    },

    /// Get the Creator Store listing for an asset
    GetAsset {
        /// Asset ID
        #[clap(short = 'A', long, value_parser)]
        asset_id: u64,

        #[clap(flatten)]
        common: Common,
    },
}

/// Access the Roblox Creator Store API
#[derive(Debug, Args)]
pub struct CreatorStore {
    #[clap(subcommand)]
    command: CreatorStoreCommands,
}

impl CreatorStore {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            CreatorStoreCommands::GetProduct { product_id, common } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client.creator_store().get_product(&product_id).await?;
                to_json(&res, common.pretty)
            }

            CreatorStoreCommands::CreateProduct {
                kind,
                asset_id,
                price,
                unpublished,
                common,
            } => {
                let mut product = CreatorStoreProduct::for_asset(kind, asset_id);
                product.base_price = Some(Money::usd_cents(parse_usd(&price)?));
                product.published = Some(!unpublished);
                let client = super::v2_client(&common.api_key)?;
                let res = client.creator_store().create_product(&product).await?;
                to_json(&res, common.pretty)
            }

            CreatorStoreCommands::UpdateProduct {
                product_id,
                price,
                published,
                common,
            } => {
                let mut product = CreatorStoreProduct::default();
                let mut mask = vec![];
                if let Some(price) = price {
                    product.base_price = Some(Money::usd_cents(parse_usd(&price)?));
                    mask.push("basePrice");
                }
                if let Some(published) = published {
                    product.published = Some(published);
                    mask.push("published");
                }
                if mask.is_empty() {
                    anyhow::bail!("nothing to update: pass --price and/or --published");
                }
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .creator_store()
                    .update_product(&product_id, &product, Some(mask.join(",")), None)
                    .await?;
                to_json(&res, common.pretty)
            }

            CreatorStoreCommands::Search {
                query,
                category,
                sort,
                descending,
                user_id,
                group_id,
                verified,
                limit,
                page_token,
                common,
            } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .creator_store()
                    .search(&SearchParams {
                        category,
                        query,
                        sort,
                        descending: descending.then_some(true),
                        creator_user_id: user_id,
                        creator_group_id: group_id,
                        only_verified_creators: verified.then_some(true),
                        max_page_size: limit,
                        page_token,
                    })
                    .await?;
                to_json(&res, common.pretty)
            }

            CreatorStoreCommands::GetAsset { asset_id, common } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client.creator_store().get_asset(asset_id).await?;
                to_json(&res, common.pretty)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parse_usd;

    #[test]
    fn usd_parsing() {
        assert_eq!(parse_usd("4.99").unwrap(), 499);
        assert_eq!(parse_usd("5").unwrap(), 500);
        assert_eq!(parse_usd("0.5").unwrap(), 50);
        assert_eq!(parse_usd("0").unwrap(), 0);
        assert!(parse_usd("1.999").is_err());
        assert!(parse_usd("abc").is_err());
    }
}
