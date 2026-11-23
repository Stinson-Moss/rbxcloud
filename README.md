# Roblox Open Cloud CLI &emsp; ![Logo](imgs/icon_32_bottom.png)
[![CI](https://github.com/Stinson-Moss/rbxcloud/actions/workflows/ci.yaml/badge.svg)](https://github.com/Stinson-Moss/rbxcloud/actions/workflows/ci.yaml)
[![Release](https://github.com/Stinson-Moss/rbxcloud/actions/workflows/release.yaml/badge.svg?event=push)](https://github.com/Stinson-Moss/rbxcloud/actions/workflows/release.yaml)

The `rbxcloud` CLI lets developers easily communicate with the Roblox Open Cloud APIs. The underlying library can also be used to build tools and applications that utilize the various APIs.

Possible use-cases:
- Deployment pipelines
- Asset management
- Live-ops
- Custom analytics
- Handling data removal requests (see [GDPR & CCPA info](https://create.roblox.com/docs/production/publishing/about-GDPR-and-CCPA))
- Debugging DataStores and OrderedDataStores

## Supported APIs

| | API v1 |
| -- | -- |
| :white_check_mark: | Assets (incl. versions, rollback, permissions) |
| :white_check_mark: | Data Stores <small>(superseded by v2)</small> |
| :white_check_mark: | Ordered Data Stores <small>(superseded by v2)</small> |
| :white_check_mark: | Messaging |
| :white_check_mark: | Place Publishing |

| | API v2 |
| -- | -- |
| :white_check_mark: | Data Stores |
| :white_check_mark: | Ordered Data Stores |
| :white_check_mark: | Memory Stores |
| :white_check_mark: | Groups (incl. join requests, roles, forums) |
| :white_check_mark: | Universes (incl. publish message) |
| :white_check_mark: | Places |
| :white_check_mark: | Instances |
| :white_check_mark: | Subscriptions |
| :white_check_mark: | Inventory |
| :white_check_mark: | User Notifications |
| :white_check_mark: | User |
| :white_check_mark: | User Restrictions |
| :white_check_mark: | Creator Store |
| :white_check_mark: | Luau Execution (incl. binary input/output) |
| :white_check_mark: | Secrets Store |
| :white_check_mark: | Game Passes |
| :white_check_mark: | Developer Products |
| :white_check_mark: | Asset Quotas |

- :white_check_mark: = Supported

The goal of this project is to support all API endpoints that Roblox provides.

## Install CLI
### mise-en-place
```sh
$ mise use github:Stinson-Moss/rbxcloud@0.18.0
```

### Aftman
Run the `aftman add` command within your project directory. This will add `rbxcloud` to the project's `aftman.toml` file (or create one if it doesn't yet exist).
```sh
$ aftman add Stinson-Moss/rbxcloud@0.18.0
```

### From Release
Download and unzip the tool for your OS from the [releases](https://github.com/Stinson-Moss/rbxcloud/releases) page. You will then need to put it in your desired location and point your path variable to the executable location.

## CLI Documentation
Visit the documentation site for information on installation and usage: https://stinson-moss.github.io/rbxcloud/

## Install Rust Library
The library built for the CLI tool is available to use directly in Rust projects. This is essentially an SDK for the Roblox Open Cloud APIs.

### Add Dependency

This fork is not published to crates.io, so add it as a git dependency in `Cargo.toml`:
```toml
[dependencies]
rbxcloud = { git = "https://github.com/Stinson-Moss/rbxcloud", tag = "v0.18.0" }
```

Alternatively, use `cargo add`.
```sh
$ cargo add rbxcloud --git https://github.com/Stinson-Moss/rbxcloud --tag v0.18.0
```

Run `cargo doc --open` in this repository (or `mise run docs:rust`) for library documentation.
