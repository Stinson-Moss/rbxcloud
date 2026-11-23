use clap::{Args, Subcommand};
use rbxcloud::rbx::{
    types::{PlaceId, UniverseId},
    v2::instance::ROOT,
};

use super::to_json;

#[derive(Debug, Args)]
struct Target {
    /// Universe ID of the experience
    #[clap(short, long, value_parser)]
    universe_id: u64,

    /// Place ID
    #[clap(short = 'i', long, value_parser)]
    place_id: u64,

    /// Instance ID (defaults to the data model root)
    #[clap(short = 'n', long, value_parser, default_value = ROOT)]
    instance_id: String,

    /// Roblox Open Cloud API Key
    #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
    api_key: String,
}

#[derive(Debug, Subcommand)]
enum InstanceCommands {
    /// Get an instance and its properties
    Get {
        #[clap(flatten)]
        target: Target,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,
    },

    /// List an instance's direct children
    Children {
        #[clap(flatten)]
        target: Target,

        /// Maximum number of children to return
        #[clap(short, long, value_parser)]
        limit: Option<u32>,

        /// Page token from a previous response's `nextPageToken`
        #[clap(short, long, value_parser)]
        cursor: Option<String>,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,
    },

    /// Print the source of a Script, LocalScript, or ModuleScript
    GetSource {
        #[clap(flatten)]
        target: Target,
    },

    /// Replace the source of a Script, LocalScript, or ModuleScript
    SetSource {
        #[clap(flatten)]
        target: Target,

        /// File containing the new source
        #[clap(short, long, value_parser, conflicts_with = "source")]
        filepath: Option<String>,

        /// The new source, inline
        #[clap(short, long, value_parser)]
        source: Option<String>,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,
    },
}

/// Read and edit the data model of a published place
#[derive(Debug, Args)]
pub struct InstanceCli {
    #[clap(subcommand)]
    command: InstanceCommands,
}

impl InstanceCli {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            InstanceCommands::Get { target, pretty } => {
                let client = super::v2_client(&target.api_key)?;
                let res = client
                    .instance(UniverseId(target.universe_id), PlaceId(target.place_id))
                    .get(&target.instance_id)
                    .await?;
                to_json(&res, pretty)
            }

            InstanceCommands::Children {
                target,
                limit,
                cursor,
                pretty,
            } => {
                let client = super::v2_client(&target.api_key)?;
                let res = client
                    .instance(UniverseId(target.universe_id), PlaceId(target.place_id))
                    .list_children(&target.instance_id, limit, cursor)
                    .await?;
                to_json(&res, pretty)
            }

            InstanceCommands::GetSource { target } => {
                let client = super::v2_client(&target.api_key)?;
                let instance = client
                    .instance(UniverseId(target.universe_id), PlaceId(target.place_id))
                    .get(&target.instance_id)
                    .await?;
                let source = instance
                    .engine_instance
                    .details
                    .as_ref()
                    .and_then(|d| d.source())
                    .ok_or_else(|| {
                        anyhow::anyhow!("instance {} is not a script", target.instance_id)
                    })?;
                Ok(Some(source.to_string()))
            }

            InstanceCommands::SetSource {
                target,
                filepath,
                source,
                pretty,
            } => {
                let source = match (filepath, source) {
                    (Some(path), _) => tokio::fs::read_to_string(&path)
                        .await
                        .map_err(|e| anyhow::anyhow!("failed to read {path}: {e}"))?,
                    (None, Some(source)) => source,
                    (None, None) => anyhow::bail!("provide --filepath or --source"),
                };
                let client = super::v2_client(&target.api_key)?;
                let res = client
                    .instance(UniverseId(target.universe_id), PlaceId(target.place_id))
                    .set_script_source(&target.instance_id, source)
                    .await?;
                to_json(&res, pretty)
            }
        }
    }
}
