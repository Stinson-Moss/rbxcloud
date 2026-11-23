# Contributing
The `rbxcloud` project is open-source and welcomes PRs. If you wish to contribute to the project, please follow the guidelines listed here.

## Purpose
The goal of the `rbxcloud` project is to provide an API and CLI tool to access Roblox's [Open Cloud](https://create.roblox.com/docs/cloud/open-cloud). As Roblox's Open Cloud matures and adds new features, the `rbxcloud` project should attempt to mirror these additions.

## Development Setup
The project's tools are managed with [mise-en-place](https://mise.jdx.dev). With mise installed, run this once in the repository:
```sh
$ mise install
```

This installs the Rust toolchain (with `clippy` and `rustfmt`) and [`uv`](https://docs.astral.sh/uv/), which runs the Python helper scripts and the docs site. Run `mise tasks` to list every command. The most common ones:

| Command | What it does |
| -- | -- |
| `mise run build` | Debug build |
| `mise run cli -- <args>` | Run the CLI from source, e.g. `mise run cli -- universe get -u 123` |
| `mise run test` | Unit tests and doctests |
| `mise run lint` | Formatting check and clippy, exactly as CI runs them |
| `mise run fix` | Apply clippy suggestions and format |
| `mise run ci` | Everything CI checks; run this before opening a PR |
| `mise run docs:cli` | Regenerate the generated CLI docs pages from `--help` |
| `mise run docs:serve` | Preview the docs site locally |
| `mise run release:check` | Pre-release checks, then prints the tag commands |

On Windows, building requires the Visual Studio C++ build tools (MSVC and the Windows SDK). Run cargo from PowerShell or cmd rather than Git Bash, whose `link` command shadows the MSVC linker.

## Live Tests
`mise run test` only runs offline tests. The live tests in `tests/live.rs` call the real Open Cloud API, which catches things offline tests can't: wrong URLs, missing permissions, and response fields Roblox has changed.

1. Create a **dedicated test experience** and publish it once, so it has a place with a `Workspace`.
2. On the [Creator Dashboard](https://create.roblox.com/dashboard/credentials), create an API key restricted to that experience, with the permissions below.
3. Copy `mise.local.toml.example` to `mise.local.toml` (gitignored) and fill in the key and IDs. Run `mise trust mise.local.toml` if mise asks.
4. Run `mise run test:live`. Append a name to run a subset, e.g. `mise run test:live -- luau`.

Each test skips itself (printing which variable to set) if an ID it needs is missing. Tests that modify data only run with `RBXCLOUD_TEST_WRITE = "1"`. They only touch keys named `rbxcloud-live-<run id>` and delete them afterwards. Nothing irreversible is tested: game passes and developer products can't be deleted, so only their read endpoints are covered.

| Test | Needs | API key permissions |
| -- | -- | -- |
| `universe_and_place` | universe, place | none beyond the experience |
| `user_and_asset_quotas` | user | `user.advanced:read`, `user.social:read`, `asset:read` |
| `group_reads` | group | `group:read` |
| `list_data_stores` | universe | `universe-datastores.control:list` |
| `instance_tree` | universe, place | `universe.place.instance:read` |
| `monetization_lists` | universe | `game-pass:read`, `developer-product:read` |
| `list_secrets` | universe | `universe.secret:read` |
| `creator_store_search` | none | `creator-store-product:read` |
| `luau_execution`, `luau_binary_input` | universe, place | `universe.place.luau-execution-session:write` |
| `data_store_roundtrip` (write) | universe | `universe-datastores.objects:*` (read, update, delete), `universe-datastores.versions:list` |
| `ordered_data_store_roundtrip` (write) | universe | `universe.ordered-data-store.scope.entry:read`, `:write` |
| `memory_store_queue` (write) | universe | `memory-store.queue:add`, `:dequeue`, `:discard` |
| `memory_store_sorted_map` (write) | universe | `memory-store.sorted-map:read`, `:write` |
| `publish_message` (write) | universe | `universe-messaging-service:publish` |
| `secret_roundtrip` (write) | universe | `universe.secret:read`, `:write` |

The `Live Tests` GitHub workflow runs the same suite weekly and on demand. It uses the repository secret `RBXCLOUD_API_KEY` and repository variables named like the `RBXCLOUD_TEST_*` settings above; with no key configured, it skips.

The `rbxcloud` project is split into two main chunks:
1. Rust library for accessing Roblox Open Cloud
2. CLI tool for accessing the Roblox Open Cloud

This separation allows `rbxcloud` to be used both as a standalone CLI tool and as a library for accessing Roblox Open Cloud.

### Rust Library
The Rust library (`src/rbx`) implements both a "low" and "high" level access into Roblox Open Cloud.

The low-level access implementations are internal to the project, and are designed to wrap around the actual HTTP Rest endpoints that Roblox provides. These Rust function APIs are stateless and are meant to provide basic error handling around the web endpoints.

The high-level access implementations are named "clients" and wrap around the low-level implementations. In Rust, these are structs that represent a category of endpoints (e.g. the MessagingService endpoints). These client structs are the publicly-facing APIs that end-users should be interacting with in Rust code.

### CLI
The CLI portion (`src/cli`) of the project allows users to access the Roblox Open Cloud from the command line.

Internally, the CLI implementation wraps around the high-level client implementations within the `rbx` library portion of the project. The CLI implementations should _not_ touch the lower-level function APIs. In other words, any CLI code should be interacting with client structs.

CLI processing is all handled by the [`clap`](https://docs.rs/clap/latest/clap/) crate.

## Opening PRs
Any PR should be accommodated by an associated GitHub Issue. A PR that does not have an associated GitHub Issue will be closed.

If at all possible, do not introduce any new dependencies into the project. Similarly, any changes to the versions of current dependencies must give justification for the version changes.

## Bug Fix PRs
When opening up a PR for a bug fix, please ensure the following:
- The associated GitHub Issue explains the bug in detail.
- The PR fixes said bug.
- The PR does not introduce new features.
- The PR explains what the new code does in order to fix the problem.
- The new code has been tested and successfully resolves the bug in question.
- Please try to not introduce any new dependencies.

## New Feature PRs
When opening up a PR for a new feature, please ensure the following:
- The associated GitHub Issue explains the new feature in detail.
- The new feature relates directly to mirroring a Roblox Open Cloud feature that is not currently supported.
- The PR does not introduce features beyond the associated GitHub Issue.
- The new code has been tested and successfully implements the new feature in question.
- Please try to not introduce any new dependencies.

## New Feature Expectations
When adding a new feature to support a new Roblox Open Cloud endpoint, please ensure that the code fits within the same structure as other features:

- The "low-level" implementation should be present within the `src/rbx/vX` directory, where "X" represents the Roblox Open Cloud resource version.
- The "high-level" client implementation should be present within the `src/rbx/mod.rs` source file. This is the main API for user code, as well as the CLI.
- The CLI implementation should be present within the `src/cli` directory, and wired up within the `src/cli/mod.rs` source file.
- Documentation has been added to the `docs/cli` directory (for commands listed in `scripts/gen_cli_docs.py`, regenerate it with `mise run docs:cli`).
- The documentation `mkdocs.yml` configuration file has been updated to include any new documentation files.

As an example, take a look at the implementation for the Universe endpoints:
- Low-level implementation: `src/rbx/v2/universe.rs`.
- High-level implementation: `src/rbx/v2/mod.rs` (search for "UniverseClient" and look at the struct and the implementation).
- CLI implementation: `src/cli/universe_cli.rs`
- CLI integration: `src/cli/mod.rs` (search for "Universe" and see its usage within the `Command` enum and the `Cli` implementation).
