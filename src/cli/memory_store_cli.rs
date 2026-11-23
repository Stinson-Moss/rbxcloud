use clap::{Args, Subcommand};
use rbxcloud::rbx::{
    types::UniverseId,
    v2::{
        memory_store::{MemoryStoreScope, SortKey, SortedMapOrder},
        PageOptions,
    },
};
use serde_json::Value;

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

fn parse_json(raw: &str) -> anyhow::Result<Value> {
    // Accept bare strings for convenience: `--data hello` is sent as "hello".
    Ok(serde_json::from_str(raw).unwrap_or_else(|_| Value::String(raw.to_string())))
}

#[derive(Debug, Subcommand)]
enum MemoryStoreCommands {
    /// Add an item to a queue
    QueueAdd {
        /// Queue name
        #[clap(short, long, value_parser)]
        queue: String,

        /// Item data as JSON (bare text is sent as a string)
        #[clap(short = 'D', long, value_parser)]
        data: String,

        /// Priority; higher priority items are read first
        #[clap(short = 'r', long, value_parser, allow_hyphen_values = true)]
        priority: Option<f64>,

        /// Seconds until the item expires
        #[clap(short, long, value_parser)]
        ttl: Option<u64>,

        #[clap(flatten)]
        common: Common,
    },

    /// Read items from a queue (they stay hidden for the invisibility window)
    QueueRead {
        /// Queue name
        #[clap(short, long, value_parser)]
        queue: String,

        /// Number of items to read (1-100)
        #[clap(short, long, value_parser)]
        count: Option<u32>,

        /// Fail unless `count` items are available
        #[clap(short = 'o', long, value_parser, default_value_t = false)]
        all_or_nothing: bool,

        /// Seconds the read items stay invisible to other readers
        #[clap(short = 'w', long, value_parser)]
        invisibility_window: Option<u64>,

        #[clap(flatten)]
        common: Common,
    },

    /// Remove items returned by a previous `queue-read`
    QueueDiscard {
        /// Queue name
        #[clap(short, long, value_parser)]
        queue: String,

        /// The `id` field from the `queue-read` response
        #[clap(short, long, value_parser)]
        read_id: String,

        #[clap(flatten)]
        common: Common,
    },

    /// List items in a sorted map
    SortedMapList {
        /// Sorted map name
        #[clap(short, long, value_parser)]
        map: String,

        /// Maximum number of items to return (1-200)
        #[clap(short = 'l', long, value_parser)]
        limit: Option<u32>,

        /// Page token from a previous response's `nextPageToken`
        #[clap(short, long, value_parser)]
        cursor: Option<String>,

        /// Sort direction
        #[clap(short, long, value_enum)]
        order: Option<SortedMapOrder>,

        /// Filter on `id` / `sortKey`, e.g. `id > "a" && id < "m"`
        #[clap(short, long, value_parser)]
        filter: Option<String>,

        #[clap(flatten)]
        common: Common,
    },

    /// Get a sorted map item
    SortedMapGet {
        /// Sorted map name
        #[clap(short, long, value_parser)]
        map: String,

        /// Item key
        #[clap(short, long, value_parser)]
        key: String,

        #[clap(flatten)]
        common: Common,
    },

    /// Create or overwrite a sorted map item
    SortedMapSet {
        /// Sorted map name
        #[clap(short, long, value_parser)]
        map: String,

        /// Item key
        #[clap(short, long, value_parser)]
        key: String,

        /// Item value as JSON (bare text is sent as a string)
        #[clap(short = 'D', long, value_parser)]
        data: String,

        /// Seconds until the item expires (max 3888000)
        #[clap(short, long, value_parser)]
        ttl: u64,

        /// Numeric sort key
        #[clap(
            short = 'n',
            long,
            value_parser,
            allow_hyphen_values = true,
            conflicts_with = "string_sort_key"
        )]
        numeric_sort_key: Option<f64>,

        /// String sort key
        #[clap(short = 'S', long, value_parser)]
        string_sort_key: Option<String>,

        /// Only update if the item's current etag matches this
        #[clap(short, long, value_parser)]
        etag: Option<String>,

        #[clap(flatten)]
        common: Common,
    },

    /// Delete a sorted map item
    SortedMapDelete {
        /// Sorted map name
        #[clap(short, long, value_parser)]
        map: String,

        /// Item key
        #[clap(short, long, value_parser)]
        key: String,

        #[clap(flatten)]
        common: Common,
    },

    /// Delete ALL queues and sorted maps in the universe
    Flush {
        /// Which scope to flush
        #[clap(short, long, value_enum, default_value_t = MemoryStoreScope::Live)]
        scope: MemoryStoreScope,

        #[clap(flatten)]
        common: Common,
    },

    /// Check the status of a flush operation
    FlushStatus {
        /// Operation ID returned by `flush`
        #[clap(short, long, value_parser)]
        operation_id: String,

        /// Scope the flush was started with
        #[clap(short, long, value_enum, default_value_t = MemoryStoreScope::Live)]
        scope: MemoryStoreScope,

        #[clap(flatten)]
        common: Common,
    },
}

/// Access the Roblox MemoryStore API
#[derive(Debug, Args)]
pub struct MemoryStore {
    #[clap(subcommand)]
    command: MemoryStoreCommands,
}

impl MemoryStore {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            MemoryStoreCommands::QueueAdd {
                queue,
                data,
                priority,
                ttl,
                common,
            } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .memory_store(UniverseId(common.universe_id))
                    .queue_add(&queue, parse_json(&data)?, priority, ttl)
                    .await?;
                to_json(&res, common.pretty)
            }

            MemoryStoreCommands::QueueRead {
                queue,
                count,
                all_or_nothing,
                invisibility_window,
                common,
            } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .memory_store(UniverseId(common.universe_id))
                    .queue_read(
                        &queue,
                        count,
                        all_or_nothing.then_some(true),
                        invisibility_window,
                    )
                    .await?;
                to_json(&res, common.pretty)
            }

            MemoryStoreCommands::QueueDiscard {
                queue,
                read_id,
                common,
            } => {
                let client = super::v2_client(&common.api_key)?;
                client
                    .memory_store(UniverseId(common.universe_id))
                    .queue_discard(&queue, &read_id)
                    .await?;
                Ok(None)
            }

            MemoryStoreCommands::SortedMapList {
                map,
                limit,
                cursor,
                order,
                filter,
                common,
            } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .memory_store(UniverseId(common.universe_id))
                    .sorted_map_list(
                        &map,
                        order,
                        PageOptions {
                            max_page_size: limit,
                            page_token: cursor,
                            filter,
                            show_deleted: None,
                        },
                    )
                    .await?;
                to_json(&res, common.pretty)
            }

            MemoryStoreCommands::SortedMapGet { map, key, common } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .memory_store(UniverseId(common.universe_id))
                    .sorted_map_get(&map, &key)
                    .await?;
                to_json(&res, common.pretty)
            }

            MemoryStoreCommands::SortedMapSet {
                map,
                key,
                data,
                ttl,
                numeric_sort_key,
                string_sort_key,
                etag,
                common,
            } => {
                let sort_key = match (numeric_sort_key, string_sort_key) {
                    (Some(n), _) => Some(SortKey::Numeric(n)),
                    (None, Some(s)) => Some(SortKey::String(s)),
                    (None, None) => None,
                };
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .memory_store(UniverseId(common.universe_id))
                    .sorted_map_set(&map, &key, parse_json(&data)?, ttl, sort_key, etag)
                    .await?;
                to_json(&res, common.pretty)
            }

            MemoryStoreCommands::SortedMapDelete { map, key, common } => {
                let client = super::v2_client(&common.api_key)?;
                client
                    .memory_store(UniverseId(common.universe_id))
                    .sorted_map_delete(&map, &key)
                    .await?;
                Ok(None)
            }

            MemoryStoreCommands::Flush { scope, common } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .memory_store(UniverseId(common.universe_id))
                    .flush(Some(scope))
                    .await?;
                to_json(&res, common.pretty)
            }

            MemoryStoreCommands::FlushStatus {
                operation_id,
                scope,
                common,
            } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .memory_store(UniverseId(common.universe_id))
                    .get_flush_operation(&operation_id, Some(scope))
                    .await?;
                to_json(&res, common.pretty)
            }
        }
    }
}
