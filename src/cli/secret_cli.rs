use clap::{Args, Subcommand};
use rbxcloud::rbx::types::UniverseId;

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
struct SecretValue {
    /// Secret ID (the name used with HttpService:GetSecret)
    #[clap(short = 'i', long, value_parser)]
    secret_id: String,

    /// Environment variable that holds the secret value
    #[clap(short = 'e', long, value_parser, conflicts_with_all = ["value", "filepath"])]
    from_env: Option<String>,

    /// File containing the secret value
    #[clap(short, long, value_parser, conflicts_with = "value")]
    filepath: Option<String>,

    /// The secret value inline (visible in shell history; prefer --from-env)
    #[clap(long, value_parser)]
    value: Option<String>,

    /// Only allow game servers to send the secret to this domain (e.g. *.example.com)
    #[clap(short, long, value_parser)]
    domain: Option<String>,
}

impl SecretValue {
    async fn read(&self) -> anyhow::Result<Vec<u8>> {
        if let Some(var) = &self.from_env {
            return std::env::var(var)
                .map(String::into_bytes)
                .map_err(|_| anyhow::anyhow!("environment variable {var} is not set"));
        }
        if let Some(path) = &self.filepath {
            return tokio::fs::read(path)
                .await
                .map_err(|e| anyhow::anyhow!("failed to read {path}: {e}"));
        }
        if let Some(value) = &self.value {
            return Ok(value.clone().into_bytes());
        }
        anyhow::bail!("provide the value with --from-env, --filepath, or --value")
    }
}

#[derive(Debug, Subcommand)]
enum SecretCommands {
    /// List secrets (values are never returned)
    List {
        /// Maximum number of secrets to return
        #[clap(short, long, value_parser)]
        limit: Option<u32>,

        /// Cursor from a previous response's `nextPageCursor`
        #[clap(short, long, value_parser)]
        cursor: Option<String>,

        #[clap(flatten)]
        common: Common,
    },

    /// Create a secret (encrypted locally before sending)
    Create {
        #[clap(flatten)]
        value: SecretValue,

        #[clap(flatten)]
        common: Common,
    },

    /// Replace a secret's value (encrypted locally before sending)
    Update {
        #[clap(flatten)]
        value: SecretValue,

        #[clap(flatten)]
        common: Common,
    },

    /// Delete a secret
    Delete {
        /// Secret ID
        #[clap(short = 'i', long, value_parser)]
        secret_id: String,

        #[clap(flatten)]
        common: Common,
    },
}

/// Manage a universe's secrets store
#[derive(Debug, Args)]
pub struct SecretCli {
    #[clap(subcommand)]
    command: SecretCommands,
}

impl SecretCli {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            SecretCommands::List {
                limit,
                cursor,
                common,
            } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .secrets(UniverseId(common.universe_id))
                    .list(limit, cursor)
                    .await?;
                to_json(&res, common.pretty)
            }

            SecretCommands::Create { value, common } => {
                let bytes = value.read().await?;
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .secrets(UniverseId(common.universe_id))
                    .create(&value.secret_id, &bytes, value.domain.as_deref())
                    .await?;
                to_json(&res, common.pretty)
            }

            SecretCommands::Update { value, common } => {
                let bytes = value.read().await?;
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .secrets(UniverseId(common.universe_id))
                    .update(&value.secret_id, &bytes, value.domain.as_deref())
                    .await?;
                to_json(&res, common.pretty)
            }

            SecretCommands::Delete { secret_id, common } => {
                let client = super::v2_client(&common.api_key)?;
                client
                    .secrets(UniverseId(common.universe_id))
                    .delete(&secret_id)
                    .await?;
                Ok(Some(format!("deleted secret {secret_id}")))
            }
        }
    }
}
