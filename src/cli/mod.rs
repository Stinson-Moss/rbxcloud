mod assets_cli;
mod datastore_cli;
mod experience_cli;
mod group_cli;
mod inventory_cli;
mod luau_execution_cli;
mod memory_store_cli;
mod messaging_cli;
mod notification_cli;
mod ordered_datastore_cli;
mod place_cli;
mod subscription_cli;
mod universe_cli;
mod user_cli;
mod user_restriction_cli;

use std::{sync::OnceLock, time::Duration};

use clap::{Parser, Subcommand};
use inventory_cli::Inventory;
use luau_execution_cli::Luau;
use memory_store_cli::MemoryStore;
use rbxcloud::rbx::{
    http::{self, HttpClient},
    v1::RbxCloud,
    v2::Client,
};
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
        value_parser,
        env = "RBXCLOUD_REQUEST_TIMEOUT",
        default_value_t = http::DEFAULT_TIMEOUT.as_secs()
    )]
    pub request_timeout: u64,

    /// Proxy URL for all requests (e.g. http://host:port or socks5://host:port).
    /// Defaults to the HTTPS_PROXY / HTTP_PROXY / ALL_PROXY environment variables.
    #[clap(long, global = true, value_parser, env = "RBXCLOUD_PROXY")]
    pub proxy: Option<String>,

    #[clap(subcommand)]
    pub command: Command,
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
}
