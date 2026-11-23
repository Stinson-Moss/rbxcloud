use base64::{engine::general_purpose::STANDARD, Engine as _};
use clap::{Args, Subcommand};
use rbxcloud::rbx::{
    buffer,
    types::{RobloxUserId, UniverseId},
    v2::{data_store::DataStoreEntry, EntryMetadata, PageOptions},
};
use serde_json::{json, Map, Value};

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
struct EntryTarget {
    /// DataStore name
    #[clap(short, long, value_parser)]
    datastore_name: String,

    /// DataStore scope (defaults to "global")
    #[clap(short, long, value_parser)]
    scope: Option<String>,

    /// The key of the entry
    #[clap(short, long, value_parser)]
    key: String,
}

#[derive(Debug, Args)]
struct Paging {
    /// Maximum number of items to return
    #[clap(short = 'l', long, value_parser)]
    limit: Option<u32>,

    /// Page token from a previous response's `nextPageToken`
    #[clap(short, long, value_parser)]
    cursor: Option<String>,
}

#[derive(Debug, Args)]
struct EntryMeta {
    /// Associated user ID (can be repeated)
    #[clap(short = 'U', long, value_parser)]
    user_ids: Option<Vec<u64>>,

    /// JSON-stringified attributes object
    #[clap(short = 't', long, value_parser)]
    attributes: Option<String>,
}

impl EntryMeta {
    fn into_metadata(self) -> anyhow::Result<EntryMetadata> {
        let attributes = match self.attributes {
            Some(raw) => Some(
                serde_json::from_str::<Map<String, Value>>(&raw)
                    .map_err(|e| anyhow::anyhow!("--attributes must be a JSON object: {e}"))?,
            ),
            None => None,
        };
        Ok(EntryMetadata {
            users: self
                .user_ids
                .map(|ids| ids.into_iter().map(RobloxUserId).collect()),
            attributes,
        })
    }
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum BufferFormat {
    /// Leave buffers as Roblox's tagged JSON objects
    Raw,
    /// Replace each buffer with its decoded bytes as base64
    Base64,
    /// Replace each buffer with its decoded bytes as hex
    Hex,
    /// Replace each buffer with its decoded bytes as UTF-8 text (lossy)
    Utf8,
}

#[derive(Debug, Subcommand)]
enum DataStoreCommands {
    /// List all DataStores in a given universe
    ListStores {
        /// Return only DataStores whose name starts with this prefix
        #[clap(short = 'r', long, value_parser)]
        prefix: Option<String>,

        /// Include DataStores scheduled for deletion
        #[clap(long, value_parser, default_value_t = false)]
        show_deleted: bool,

        #[clap(flatten)]
        paging: Paging,

        #[clap(flatten)]
        common: Common,
    },

    /// List entries in a DataStore
    List {
        /// DataStore name
        #[clap(short, long, value_parser)]
        datastore_name: String,

        /// DataStore scope (defaults to "global")
        #[clap(short, long, value_parser)]
        scope: Option<String>,

        /// Return only keys that start with this prefix
        #[clap(short = 'r', long, value_parser)]
        prefix: Option<String>,

        /// Include entries marked as deleted
        #[clap(long, value_parser, default_value_t = false)]
        show_deleted: bool,

        #[clap(flatten)]
        paging: Paging,

        #[clap(flatten)]
        common: Common,
    },

    /// Get a DataStore entry (value plus metadata)
    Get {
        #[clap(flatten)]
        target: EntryTarget,

        /// Read a specific revision instead of the latest value
        #[clap(short = 'R', long, value_parser)]
        revision: Option<String>,

        /// Print only the entry's value
        #[clap(short = 'o', long, value_parser, default_value_t = false)]
        only_value: bool,

        /// How to present Luau buffer values
        #[clap(short = 'b', long, value_enum, default_value_t = BufferFormat::Raw)]
        buffers: BufferFormat,

        #[clap(flatten)]
        common: Common,
    },

    /// Set (or create) the value of a DataStore entry
    Set {
        #[clap(flatten)]
        target: EntryTarget,

        /// JSON-stringified value
        #[clap(short = 'D', long, value_parser)]
        data: String,

        /// Only update if the entry's current etag matches this
        #[clap(short = 'e', long, value_parser)]
        etag: Option<String>,

        /// Fail if the entry already exists
        #[clap(short = 'x', long, value_parser, default_value_t = false)]
        exclusive_create: bool,

        #[clap(flatten)]
        meta: EntryMeta,

        #[clap(flatten)]
        common: Common,
    },

    /// Increment (or create) the numeric value of a DataStore entry
    Increment {
        #[clap(flatten)]
        target: EntryTarget,

        /// The amount by which the entry should be incremented
        #[clap(short, long, value_parser, allow_hyphen_values = true)]
        increment_by: f64,

        #[clap(flatten)]
        meta: EntryMeta,

        #[clap(flatten)]
        common: Common,
    },

    /// Delete a DataStore entry
    Delete {
        #[clap(flatten)]
        target: EntryTarget,

        #[clap(flatten)]
        common: Common,
    },

    /// List the revisions of a DataStore entry
    ListRevisions {
        #[clap(flatten)]
        target: EntryTarget,

        /// Only revisions created at or after this time (RFC 3339)
        #[clap(long, value_parser)]
        start_time: Option<String>,

        /// Only revisions created at or before this time (RFC 3339)
        #[clap(long, value_parser)]
        end_time: Option<String>,

        #[clap(flatten)]
        paging: Paging,

        #[clap(flatten)]
        common: Common,
    },

    /// Schedule a DataStore for deletion (recoverable with `undelete-store`)
    DeleteStore {
        /// DataStore name
        #[clap(short, long, value_parser)]
        datastore_name: String,

        #[clap(flatten)]
        common: Common,
    },

    /// Restore a DataStore scheduled for deletion
    UndeleteStore {
        /// DataStore name
        #[clap(short, long, value_parser)]
        datastore_name: String,

        #[clap(flatten)]
        common: Common,
    },

    /// Snapshot all DataStores in the universe (at most once per UTC day)
    Snapshot {
        #[clap(flatten)]
        common: Common,
    },
}

/// Access the Roblox DataStore API (Open Cloud v2)
#[derive(Debug, Args)]
pub struct DataStore {
    #[clap(subcommand)]
    command: DataStoreCommands,
}

fn prefix_filter(prefix: Option<String>) -> Option<String> {
    prefix.map(|p| format!("id.startsWith({})", json!(p)))
}

fn time_filter(start: Option<String>, end: Option<String>) -> Option<String> {
    let parts: Vec<String> = [
        start.map(|t| format!("revision_create_time >= {t}")),
        end.map(|t| format!("revision_create_time <= {t}")),
    ]
    .into_iter()
    .flatten()
    .collect();
    (!parts.is_empty()).then(|| parts.join(" && "))
}

fn present_buffers(value: Value, format: BufferFormat) -> anyhow::Result<Value> {
    let convert: fn(Vec<u8>) -> Value = match format {
        BufferFormat::Raw => return Ok(value),
        BufferFormat::Base64 => |b| Value::String(STANDARD.encode(b)),
        BufferFormat::Hex => |b| Value::String(b.iter().map(|x| format!("{x:02x}")).collect()),
        BufferFormat::Utf8 => |b| Value::String(String::from_utf8_lossy(&b).into_owned()),
    };
    Ok(buffer::map_buffers(&value, &convert)?)
}

fn entry_output(
    mut entry: DataStoreEntry,
    only_value: bool,
    buffers: BufferFormat,
    pretty: bool,
) -> anyhow::Result<Option<String>> {
    if let Some(value) = entry.value.take() {
        entry.value = Some(present_buffers(value, buffers)?);
    }
    if only_value {
        to_json(&entry.value.unwrap_or(Value::Null), pretty)
    } else {
        to_json(&entry, pretty)
    }
}

impl DataStore {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            DataStoreCommands::ListStores {
                prefix,
                show_deleted,
                paging,
                common,
            } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .data_store(UniverseId(common.universe_id))
                    .list_data_stores(PageOptions {
                        max_page_size: paging.limit,
                        page_token: paging.cursor,
                        filter: prefix_filter(prefix),
                        show_deleted: show_deleted.then_some(true),
                    })
                    .await?;
                to_json(&res, common.pretty)
            }

            DataStoreCommands::List {
                datastore_name,
                scope,
                prefix,
                show_deleted,
                paging,
                common,
            } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .data_store(UniverseId(common.universe_id))
                    .list_entries(
                        &datastore_name,
                        scope.as_deref(),
                        PageOptions {
                            max_page_size: paging.limit,
                            page_token: paging.cursor,
                            filter: prefix_filter(prefix),
                            show_deleted: show_deleted.then_some(true),
                        },
                    )
                    .await?;
                to_json(&res, common.pretty)
            }

            DataStoreCommands::Get {
                target,
                revision,
                only_value,
                buffers,
                common,
            } => {
                let client = super::v2_client(&common.api_key)?;
                let entry_id = match revision {
                    Some(rev) => format!("{}@{rev}", target.key),
                    None => target.key,
                };
                let entry = client
                    .data_store(UniverseId(common.universe_id))
                    .get_entry(&target.datastore_name, target.scope.as_deref(), &entry_id)
                    .await?;
                entry_output(entry, only_value, buffers, common.pretty)
            }

            DataStoreCommands::Set {
                target,
                data,
                etag,
                exclusive_create,
                meta,
                common,
            } => {
                let value: Value = serde_json::from_str(&data)
                    .map_err(|e| anyhow::anyhow!("--data must be valid JSON: {e}"))?;
                let metadata = meta.into_metadata()?;
                let client = super::v2_client(&common.api_key)?;
                let ds = client.data_store(UniverseId(common.universe_id));
                let scope = target.scope.as_deref();
                let entry = if exclusive_create {
                    ds.create_entry(&target.datastore_name, scope, &target.key, value, metadata)
                        .await?
                } else {
                    ds.set_entry(
                        &target.datastore_name,
                        scope,
                        &target.key,
                        value,
                        etag,
                        metadata,
                    )
                    .await?
                };
                to_json(&entry, common.pretty)
            }

            DataStoreCommands::Increment {
                target,
                increment_by,
                meta,
                common,
            } => {
                let metadata = meta.into_metadata()?;
                let client = super::v2_client(&common.api_key)?;
                let entry = client
                    .data_store(UniverseId(common.universe_id))
                    .increment_entry(
                        &target.datastore_name,
                        target.scope.as_deref(),
                        &target.key,
                        increment_by,
                        metadata,
                    )
                    .await?;
                to_json(&entry, common.pretty)
            }

            DataStoreCommands::Delete { target, common } => {
                let client = super::v2_client(&common.api_key)?;
                client
                    .data_store(UniverseId(common.universe_id))
                    .delete_entry(&target.datastore_name, target.scope.as_deref(), &target.key)
                    .await?;
                Ok(None)
            }

            DataStoreCommands::ListRevisions {
                target,
                start_time,
                end_time,
                paging,
                common,
            } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .data_store(UniverseId(common.universe_id))
                    .list_entry_revisions(
                        &target.datastore_name,
                        target.scope.as_deref(),
                        &target.key,
                        PageOptions {
                            max_page_size: paging.limit,
                            page_token: paging.cursor,
                            filter: time_filter(start_time, end_time),
                            show_deleted: None,
                        },
                    )
                    .await?;
                to_json(&res, common.pretty)
            }

            DataStoreCommands::DeleteStore {
                datastore_name,
                common,
            } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .data_store(UniverseId(common.universe_id))
                    .delete_data_store(&datastore_name)
                    .await?;
                to_json(&res, common.pretty)
            }

            DataStoreCommands::UndeleteStore {
                datastore_name,
                common,
            } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .data_store(UniverseId(common.universe_id))
                    .undelete_data_store(&datastore_name)
                    .await?;
                to_json(&res, common.pretty)
            }

            DataStoreCommands::Snapshot { common } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .data_store(UniverseId(common.universe_id))
                    .snapshot()
                    .await?;
                to_json(&res, common.pretty)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters() {
        assert_eq!(
            prefix_filter(Some("user_".into())).as_deref(),
            Some(r#"id.startsWith("user_")"#)
        );
        assert_eq!(
            time_filter(Some("2024-01-01T00:00:00Z".into()), None).as_deref(),
            Some("revision_create_time >= 2024-01-01T00:00:00Z")
        );
        assert_eq!(time_filter(None, None), None);
    }

    #[test]
    fn buffer_presentation() {
        let value = json!({"blob": buffer::encode(b"hi")});
        assert_eq!(
            present_buffers(value.clone(), BufferFormat::Hex).unwrap(),
            json!({"blob": "6869"})
        );
        assert_eq!(
            present_buffers(value.clone(), BufferFormat::Utf8).unwrap(),
            json!({"blob": "hi"})
        );
        assert_eq!(
            present_buffers(value.clone(), BufferFormat::Raw).unwrap(),
            value
        );
    }
}
