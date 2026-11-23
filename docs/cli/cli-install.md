# Install

## Install CLI

There are a few different ways to install the `rbxcloud` CLI.

### [mise-en-place](https://mise.jdx.dev) <small>(Preferred)</small>
Run `mise use` within your project directory. This adds `rbxcloud` to the project's `mise.toml` file (creating it if needed) and installs it from the GitHub releases.
```sh
$ mise use github:Stinson-Moss/rbxcloud@0.18.0
```

Omit the version (`github:Stinson-Moss/rbxcloud`) to use the latest release. Anyone else working in the project then only needs to run `mise install`.

### [Aftman](https://github.com/LPGhatguy/aftman)
Run the `aftman add` command within your project directory. This will add `rbxcloud` to the project's `aftman.toml` file (or create one if it doesn't yet exist).
```sh
$ aftman add Stinson-Moss/rbxcloud@0.18.0
```

Next, run `aftman install` to install `rbxcloud`.

### [Foreman](https://github.com/Roblox/foreman)
Add `rbxcloud` under the `[tools]` section of your `foreman.toml` file.
```toml
# foreman.toml
[tools]
rbxcloud = { github = "Stinson-Moss/rbxcloud", version = "0.18.0" }
```

Next, run `foreman install` to install `rbxcloud`.

??? info "Legacy"
	mise is preferred over Aftman and Foreman. For more information, see the [mise-en-place](https://mise.jdx.dev) documentation.

### [Releases](https://github.com/Stinson-Moss/rbxcloud/releases)
A few prebuilt binaries are compiled and attached to each release.

1. Go to the [`releases`](https://github.com/Stinson-Moss/rbxcloud/releases) page
1. Navigate to the desired version (e.g. the [latest version](https://github.com/Stinson-Moss/rbxcloud/releases/latest))
1. Download and unzip the desired executable
1. Relocate the executable to a more permanent place
1. Change your OS path variable to point to the executable

## Verify
Run the tool with the `--version` flag to verify the installation succeeded.
```sh
$ rbxcloud --version
```
