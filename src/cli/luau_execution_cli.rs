use clap::{Args, Subcommand};
use rbxcloud::rbx::{
    types::{PlaceId, UniverseId},
    v2::luau_execution::{LuauExecutionOutcome, LuauExecutionTaskLogView},
};
use tokio::fs;

#[derive(Debug, Subcommand)]
pub enum LuauExecutionCommands {
    /// Executes Luau code on Roblox
    Execute {
        /// Universe ID of the experience
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Place ID of the experience
        #[clap(short = 'i', long, value_parser)]
        place_id: u64,

        /// Version ID of the experience
        #[clap(short = 'r', long, value_parser)]
        version_id: Option<String>,

        /// Script source code
        #[clap(short, long, value_parser)]
        script: Option<String>,

        /// Script source code file
        #[clap(short, long, value_parser)]
        filepath: Option<String>,

        /// Execution timeout
        #[clap(short, long, value_parser)]
        timeout: Option<String>,

        /// File to pass to the script as `({...})[1].BinaryInput` (max 100 MiB)
        #[clap(short, long, value_parser)]
        binary_input: Option<String>,

        /// Allow the script to return a `BinaryOutput` buffer (fetch it with
        /// `get-task --save-binary-output`)
        #[clap(long, value_parser, default_value_t = false)]
        binary_output: bool,

        /// Wait for the task to finish and print the final task (exits with an
        /// error if the script fails)
        #[clap(short = 'W', long, value_parser, default_value_t = false)]
        wait: bool,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Gets information on a previously executed task
    GetTask {
        /// Universe ID of the experience
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Place ID of the experience
        #[clap(short = 'i', long, value_parser)]
        place_id: u64,

        /// Version ID of the experience
        #[clap(short = 'r', long, value_parser)]
        version_id: Option<String>,

        /// Luau execution session ID
        #[clap(short, long, value_parser)]
        session_id: String,

        /// Luau execution task ID
        #[clap(short, long, value_parser)]
        task_id: String,

        /// Download the task's binary output to this file
        #[clap(long, value_parser)]
        save_binary_output: Option<String>,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Retrieves logs on a previously executed task
    GetLogs {
        /// Universe ID of the experience
        #[clap(short, long, value_parser)]
        universe_id: u64,

        /// Place ID of the experience
        #[clap(short = 'i', long, value_parser)]
        place_id: u64,

        /// Version ID of the experience
        #[clap(short = 'r', long, value_parser)]
        version_id: Option<String>,

        /// Luau execution session ID
        #[clap(short, long, value_parser)]
        session_id: String,

        /// Luau execution task ID
        #[clap(short, long, value_parser)]
        task_id: String,

        /// Max page size
        #[clap(short, long, value_parser)]
        max_page_size: Option<u32>,

        /// Next page token
        #[clap(short = 'n', long, value_parser)]
        page_token: Option<String>,

        /// Log view type
        #[clap(short = 'w', long, value_enum)]
        view: Option<LuauExecutionTaskLogView>,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },
}

#[derive(Debug, Args)]
pub struct Luau {
    #[clap(subcommand)]
    command: LuauExecutionCommands,
}

impl Luau {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            LuauExecutionCommands::Execute {
                universe_id,
                place_id,
                version_id,
                script,
                filepath,
                timeout,
                binary_input,
                binary_output,
                wait,
                pretty,
                api_key,
            } => {
                let client = super::v2_client(&api_key)?;

                let luau = client.luau(UniverseId(universe_id), PlaceId(place_id), version_id);

                let mut script_source: Option<String> = None;
                if script.is_some() {
                    script_source = script;
                }

                if script_source.is_none() {
                    if let Some(path) = filepath {
                        let src_bytes = fs::read(path).await?;
                        script_source = Some(String::from_utf8(src_bytes)?);
                    }
                }

                let src = script_source.expect("script or filepath must be set");

                let binary_input =
                    match binary_input {
                        Some(path) => Some(fs::read(&path).await.map_err(|e| {
                            anyhow::anyhow!("failed to read binary input {path}: {e}")
                        })?),
                        None => None,
                    };
                let res = luau
                    .create_task_with_binary(src, timeout, binary_input, binary_output)
                    .await;
                let res = match (res, wait) {
                    (Ok(task), true) => {
                        let task = luau.wait(task).await?;
                        if let LuauExecutionOutcome::Failed(err) = task.outcome() {
                            anyhow::bail!("Luau task failed: {err}");
                        }
                        Ok(task)
                    }
                    (res, _) => res,
                };
                match res {
                    Ok(data) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&data)?
                        } else {
                            serde_json::to_string(&data)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }

            LuauExecutionCommands::GetTask {
                universe_id,
                place_id,
                version_id,
                session_id,
                task_id,
                save_binary_output,
                pretty,
                api_key,
            } => {
                let client = super::v2_client(&api_key)?;

                let luau = client.luau(UniverseId(universe_id), PlaceId(place_id), version_id);

                let res = luau.get_task(session_id, task_id).await;
                if let (Ok(task), Some(path)) = (&res, &save_binary_output) {
                    let bytes = luau.download_binary_output(task).await?;
                    fs::write(path, &bytes)
                        .await
                        .map_err(|e| anyhow::anyhow!("failed to write {path}: {e}"))?;
                    eprintln!("saved {} bytes of binary output to {path}", bytes.len());
                }
                match res {
                    Ok(data) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&data)?
                        } else {
                            serde_json::to_string(&data)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }

            LuauExecutionCommands::GetLogs {
                universe_id,
                place_id,
                version_id,
                session_id,
                task_id,
                max_page_size,
                page_token,
                view,
                pretty,
                api_key,
            } => {
                let client = super::v2_client(&api_key)?;

                let luau = client.luau(UniverseId(universe_id), PlaceId(place_id), version_id);

                let res = luau
                    .get_logs(
                        session_id,
                        task_id,
                        view.unwrap_or(LuauExecutionTaskLogView::Flat),
                        max_page_size,
                        page_token,
                    )
                    .await;
                match res {
                    Ok(data) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&data)?
                        } else {
                            serde_json::to_string(&data)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }
        }
    }
}
