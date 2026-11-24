mod assets_cli;
mod creator_store_cli;
mod datastore_cli;
mod experience_cli;
mod group_cli;
mod instance_cli;
mod inventory_cli;
mod luau_execution_cli;
mod memory_store_cli;
mod messaging_cli;
mod monetization_cli;
mod notification_cli;
mod ordered_datastore_cli;
mod place_cli;
mod secret_cli;
mod subscription_cli;
mod universe_cli;
mod user_cli;
mod user_restriction_cli;

use std::{sync::OnceLock, time::Duration};

use clap::{Parser, Subcommand};
use creator_store_cli::CreatorStore;
use instance_cli::InstanceCli;
use inventory_cli::Inventory;
use luau_execution_cli::Luau;
use memory_store_cli::MemoryStore;
use monetization_cli::{DeveloperProductCli, GamePassCli};
use rbxcloud::rbx::{
    http::{self, HttpClient},
    v1::RbxCloud,
    v2::Client,
};
use secret_cli::SecretCli;
use universe_cli::Universe;
use user_cli::User;
use user_restriction_cli::UserRestriction;

use self::{
    assets_cli::Assets, datastore_cli::DataStore, experience_cli::Experience, group_cli::Group,
    messaging_cli::Messaging, notification_cli::Notification,
    ordered_datastore_cli::OrderedDataStore, place_cli::Place, subscription_cli::Subscription,
};

#[derive(Debug, Parser)]
#[clap(name = "rbxcloud", version)]
pub(crate) struct Cli {
    /// Maximum seconds to wait for each request to Roblox (0 = no limit)
    #[clap(
        long,
        global = true,
        display_order = 900,
        value_parser,
        env = "RBXCLOUD_REQUEST_TIMEOUT",
        default_value_t = http::DEFAULT_TIMEOUT.as_secs()
    )]
    pub request_timeout: u64,

    /// Proxy URL for all requests (e.g. http://host:port or socks5://host:port).
    /// Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables.
    #[clap(
        long,
        global = true,
        display_order = 901,
        value_parser,
        env = "RBXCLOUD_PROXY"
    )]
    pub proxy: Option<String>,

    /// Show environment variable values (e.g. API keys) in help output.
    /// Hidden by default to avoid leaking secrets.
    #[clap(
        long,
        global = true,
        display_order = 902,
        action = clap::ArgAction::SetTrue,
        value_parser = clap::builder::FalseyValueParser::new(),
        env = "RBXCLOUD_SHOW_ENV_VALUES"
    )]
    #[allow(dead_code)] // read by `show_env_values_requested` before parsing
    pub show_env_values: bool,

    #[clap(subcommand)]
    pub command: Command,
}

/// Whether `--show-env-values` (or a truthy `RBXCLOUD_SHOW_ENV_VALUES`) was given.
///
/// Help is rendered while clap parses, so this has to be decided by scanning the
/// raw arguments before the command is built.
pub(crate) fn show_env_values_requested() -> bool {
    let flag = std::env::args_os()
        .skip(1)
        .take_while(|a| a != "--")
        .any(|a| a == "--show-env-values");
    let env = std::env::var("RBXCLOUD_SHOW_ENV_VALUES").is_ok_and(|v| {
        !matches!(
            v.trim().to_ascii_lowercase().as_str(),
            "" | "0" | "n" | "no" | "f" | "false" | "off"
        )
    });
    flag || env
}

/// Recursively mark every argument of `cmd` and its subcommands to hide env values.
pub(crate) fn hide_all_env_values(cmd: clap::Command) -> clap::Command {
    let names: Vec<String> = cmd
        .get_subcommands()
        .map(|s| s.get_name().to_owned())
        .collect();
    let mut cmd = cmd.mut_args(|a| a.hide_env_values(true));
    for name in names {
        cmd = cmd.mut_subcommand(name, hide_all_env_values);
    }
    cmd
}

#[derive(Debug, Clone)]
struct HttpOptions {
    request_timeout: u64,
    proxy: Option<String>,
}

static HTTP_OPTIONS: OnceLock<HttpOptions> = OnceLock::new();

/// Build an HTTP client using the global CLI options.
pub(crate) fn http_client(api_key: &str) -> anyhow::Result<HttpClient> {
    let opts = HTTP_OPTIONS.get().cloned().unwrap_or(HttpOptions {
        request_timeout: http::DEFAULT_TIMEOUT.as_secs(),
        proxy: None,
    });
    let mut builder = HttpClient::builder(api_key);
    builder = match opts.request_timeout {
        0 => builder.no_timeout(),
        secs => builder.timeout(Duration::from_secs(secs)),
    };
    if let Some(proxy) = opts.proxy {
        builder = builder.proxy(proxy);
    }
    Ok(builder.build()?)
}

/// Build a v1 API client using the global CLI options.
pub(crate) fn v1_client(api_key: &str) -> anyhow::Result<RbxCloud> {
    Ok(RbxCloud::with_http(http_client(api_key)?))
}

/// Build a v2 API client using the global CLI options.
pub(crate) fn v2_client(api_key: &str) -> anyhow::Result<Client> {
    Ok(Client::with_http(http_client(api_key)?))
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Access the Roblox Assets API
    Assets(Assets),

    /// Access the Roblox Experience API
    Experience(Experience),

    /// Read and edit the data model of a published place
    Instance(InstanceCli),

    /// Manage Creator Store products and search the store
    CreatorStore(CreatorStore),

    /// Manage a universe's secrets store
    Secret(SecretCli),

    /// Manage a universe's game passes
    GamePass(GamePassCli),

    /// Manage a universe's developer products
    DeveloperProduct(DeveloperProductCli),

    /// Access the Roblox Messaging API
    Messaging(Messaging),

    /// Access the Roblox DataStore API (v2)
    Datastore(DataStore),

    /// Access the Roblox OrderedDataStore API (v2)
    OrderedDatastore(OrderedDataStore),

    /// Access the Roblox MemoryStore API (queues, sorted maps, flush)
    MemoryStore(MemoryStore),

    /// Access the Roblox user inventory API
    Inventory(Inventory),

    /// Access the Roblox Group API
    Group(Group),

    /// Execute Luau scripts against a place
    Luau(Luau),

    /// Access the Roblox Subscription API
    Subscription(Subscription),

    /// Access the Roblox Notification API
    Notification(Notification),

    /// Access the Roblox Place API
    Place(Place),

    /// Access the Roblox Universe API
    Universe(Universe),

    /// Access the Roblox User API
    User(User),

    /// Access to the Roblox User Restriction API
    UserRestriction(UserRestriction),
}

impl Cli {
    pub(crate) async fn run(self) -> anyhow::Result<Option<String>> {
        let _ = HTTP_OPTIONS.set(HttpOptions {
            request_timeout: self.request_timeout,
            proxy: self.proxy.clone(),
        });
        match self.command {
            Command::Assets(command) => command.run().await,
            Command::Experience(command) => command.run().await,
            Command::Instance(command) => command.run().await,
            Command::CreatorStore(command) => command.run().await,
            Command::Secret(command) => command.run().await,
            Command::GamePass(command) => command.run().await,
            Command::DeveloperProduct(command) => command.run().await,
            Command::Messaging(command) => command.run().await,
            Command::Datastore(command) => command.run().await,
            Command::OrderedDatastore(command) => command.run().await,
            Command::MemoryStore(command) => command.run().await,
            Command::Group(command) => command.run().await,
            Command::Inventory(command) => command.run().await,
            Command::Luau(command) => command.run().await,
            Command::Subscription(command) => command.run().await,
            Command::Notification(command) => command.run().await,
            Command::Place(command) => command.run().await,
            Command::Universe(command) => command.run().await,
            Command::User(command) => command.run().await,
            Command::UserRestriction(command) => command.run().await,
        }
    }
}

/// Serialize a response as compact or pretty JSON for printing.
pub(crate) fn to_json<T: serde::Serialize>(
    value: &T,
    pretty: bool,
) -> anyhow::Result<Option<String>> {
    Ok(Some(if pretty {
        serde_json::to_string_pretty(value)?
    } else {
        serde_json::to_string(value)?
    }))
}

#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::CommandFactory;

    /// Catches conflicting flags (e.g. two `-p` shorts) across every subcommand.
    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    /// Split a documented command line into arguments, honoring single and
    /// double quotes.
    fn split_args(line: &str) -> Vec<String> {
        let mut args = vec![];
        let mut current = String::new();
        let mut in_arg = false;
        let mut quote: Option<char> = None;
        let mut chars = line.chars();
        while let Some(c) = chars.next() {
            match (quote, c) {
                (Some(q), c) if c == q => quote = None,
                (Some('"'), '\\') => current.extend(chars.next()),
                (Some(_), c) => current.push(c),
                (None, '"' | '\'') => {
                    quote = Some(c);
                    in_arg = true;
                }
                (None, c) if c.is_whitespace() => {
                    if in_arg {
                        args.push(std::mem::take(&mut current));
                        in_arg = false;
                    }
                }
                (None, c) => {
                    current.push(c);
                    in_arg = true;
                }
            }
        }
        if in_arg {
            args.push(current);
        }
        args
    }

    /// Every `$ rbxcloud ...` example in the CLI docs must parse.
    #[test]
    fn documented_examples_parse() {
        use clap::{error::ErrorKind, Parser};

        let docs = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/cli");
        let mut checked = 0;
        let mut failures = vec![];
        for entry in std::fs::read_dir(&docs).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|e| e.to_str()) != Some("md") {
                continue;
            }
            let text = std::fs::read_to_string(&path).unwrap();
            for line in text.lines() {
                let Some(cmd) = line.trim().strip_prefix("$ ") else {
                    continue;
                };
                let cmd = cmd.trim();
                if !cmd.starts_with("rbxcloud ") {
                    continue;
                }
                checked += 1;
                let result = Cli::try_parse_from(split_args(cmd));
                if let Err(e) = result.as_ref().map(|_| ()).or_else(|e| match e.kind() {
                    // `--help` / `--version` are valid invocations.
                    ErrorKind::DisplayHelp | ErrorKind::DisplayVersion => Ok(()),
                    _ => Err(e),
                }) {
                    let name = path.file_name().unwrap().to_string_lossy();
                    let first_line = e.to_string().lines().next().unwrap_or_default().to_string();
                    failures.push(format!("{name}: `{cmd}`\n    {first_line}"));
                }
            }
        }
        assert!(checked > 0, "no examples found in {}", docs.display());
        assert!(
            failures.is_empty(),
            "{} of {checked} documented examples don't parse:\n{}",
            failures.len(),
            failures.join("\n")
        );
    }

    #[test]
    fn split_args_handles_quotes() {
        assert_eq!(
            split_args(r#"rbxcloud x -D '{"a":1}' -n "100 Coins" -f "user == 'users/1'""#),
            vec![
                "rbxcloud",
                "x",
                "-D",
                r#"{"a":1}"#,
                "-n",
                "100 Coins",
                "-f",
                "user == 'users/1'"
            ]
        );
    }
}
