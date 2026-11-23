use clap::{Args, Subcommand};

use rbxcloud::rbx::types::UniverseId;

#[derive(Debug, Subcommand)]
pub enum MessagingCommands {
    /// Publish a message
    Publish {
        /// Message topic
        #[clap(short, long, value_parser)]
        topic: String,

        /// Message to send
        #[clap(short, long, value_parser)]
        message: String,

        /// Universe ID of the experience
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },
}

#[derive(Debug, Args)]
pub struct Messaging {
    #[clap(subcommand)]
    command: MessagingCommands,
}

impl Messaging {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            MessagingCommands::Publish {
                topic,
                message,
                universe_id,
                api_key,
            } => {
                let client = super::v2_client(&api_key)?;
                let universe = client.universe(UniverseId(universe_id));
                let res = universe.publish_message(&topic, &message).await;
                match res {
                    Ok(()) => Ok(Some(format!("published message to topic {topic}"))),
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }
        }
    }
}
