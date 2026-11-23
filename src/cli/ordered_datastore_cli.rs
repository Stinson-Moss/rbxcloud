use clap::{Args, Subcommand};
use rbxcloud::rbx::{
    types::UniverseId,
    v2::{ordered_data_store::OrderedDataStoreSortOrder, PageOptions},
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

#[derive(Debug, Args)]
struct Target {
    /// OrderedDataStore name
    #[clap(short, long, value_parser)]
    datastore_name: String,

    /// DataStore scope (defaults to "global")
    #[clap(short, long, value_parser)]
    scope: Option<String>,
}

#[derive(Debug, Subcommand)]
enum OrderedDataStoreCommands {
    /// List entries
    List {
        #[clap(flatten)]
        target: Target,

        /// Maximum number of items to return per page
        #[clap(short, long, value_parser)]
        max_page_size: Option<u32>,

        /// Page token from a previous response's `nextPageToken`
        #[clap(short = 't', long, value_parser)]
        page_token: Option<String>,

        /// Sort by value in this direction
        #[clap(short, long, value_enum)]
        order_by: Option<OrderedDataStoreSortOrder>,

        /// Range of values to return, e.g. "entry >= 10 && entry <= 50"
        #[clap(short, long, value_parser)]
        filter: Option<String>,

        #[clap(flatten)]
        common: Common,
    },

    /// Create an entry (fails if it already exists)
    Create {
        #[clap(flatten)]
        target: Target,

        /// The ID of the entry
        #[clap(short, long, value_parser)]
        id: String,

        /// The value of the entry
        #[clap(short, long, value_parser, allow_hyphen_values = true)]
        value: i64,

        #[clap(flatten)]
        common: Common,
    },

    /// Get an entry
    Get {
        #[clap(flatten)]
        target: Target,

        /// The ID of the entry
        #[clap(short, long, value_parser)]
        id: String,

        #[clap(flatten)]
        common: Common,
    },

    /// Delete an entry
    Delete {
        #[clap(flatten)]
        target: Target,

        /// The ID of the entry
        #[clap(short, long, value_parser)]
        id: String,

        #[clap(flatten)]
        common: Common,
    },

    /// Update an entry's value
    Update {
        #[clap(flatten)]
        target: Target,

        /// The ID of the entry
        #[clap(short, long, value_parser)]
        id: String,

        /// The new value of the entry
        #[clap(short, long, value_parser, allow_hyphen_values = true)]
        value: i64,

        /// Create the entry if it does not exist
        #[clap(short = 'm', long, value_parser)]
        allow_missing: Option<bool>,

        #[clap(flatten)]
        common: Common,
    },

    /// Increment an entry's value (creating it if needed)
    Increment {
        #[clap(flatten)]
        target: Target,

        /// The ID of the entry
        #[clap(short, long, value_parser)]
        id: String,

        /// The amount by which the entry should be incremented
        #[clap(short = 'n', long, value_parser, allow_hyphen_values = true)]
        increment: i64,

        #[clap(flatten)]
        common: Common,
    },
}

/// Access the Roblox OrderedDataStore API (Open Cloud v2)
#[derive(Debug, Args)]
pub struct OrderedDataStore {
    #[clap(subcommand)]
    command: OrderedDataStoreCommands,
}

impl OrderedDataStore {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            OrderedDataStoreCommands::List {
                target,
                max_page_size,
                page_token,
                order_by,
                filter,
                common,
            } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .ordered_data_store(UniverseId(common.universe_id))
                    .list_entries(
                        &target.datastore_name,
                        target.scope.as_deref(),
                        order_by,
                        PageOptions {
                            max_page_size,
                            page_token,
                            filter,
                            show_deleted: None,
                        },
                    )
                    .await?;
                to_json(&res, common.pretty)
            }

            OrderedDataStoreCommands::Create {
                target,
                id,
                value,
                common,
            } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .ordered_data_store(UniverseId(common.universe_id))
                    .create_entry(&target.datastore_name, target.scope.as_deref(), &id, value)
                    .await?;
                to_json(&res, common.pretty)
            }

            OrderedDataStoreCommands::Get { target, id, common } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .ordered_data_store(UniverseId(common.universe_id))
                    .get_entry(&target.datastore_name, target.scope.as_deref(), &id)
                    .await?;
                to_json(&res, common.pretty)
            }

            OrderedDataStoreCommands::Delete { target, id, common } => {
                let client = super::v2_client(&common.api_key)?;
                client
                    .ordered_data_store(UniverseId(common.universe_id))
                    .delete_entry(&target.datastore_name, target.scope.as_deref(), &id)
                    .await?;
                Ok(None)
            }

            OrderedDataStoreCommands::Update {
                target,
                id,
                value,
                allow_missing,
                common,
            } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .ordered_data_store(UniverseId(common.universe_id))
                    .update_entry(
                        &target.datastore_name,
                        target.scope.as_deref(),
                        &id,
                        value,
                        allow_missing,
                    )
                    .await?;
                to_json(&res, common.pretty)
            }

            OrderedDataStoreCommands::Increment {
                target,
                id,
                increment,
                common,
            } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .ordered_data_store(UniverseId(common.universe_id))
                    .increment_entry(
                        &target.datastore_name,
                        target.scope.as_deref(),
                        &id,
                        increment,
                    )
                    .await?;
                to_json(&res, common.pretty)
            }
        }
    }
}
