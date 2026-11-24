# Rust Library Installation

## Add Dependency

This fork is not published to crates.io, so add `rbxcloud` as a git dependency in `Cargo.toml`:
```toml
[dependencies]
rbxcloud = { git = "https://github.com/Stinson-Moss/rbxcloud", tag = "v0.19.0" }
```

Alternatively, use `cargo add`.
```sh
$ cargo add rbxcloud --git https://github.com/Stinson-Moss/rbxcloud --tag v0.19.0
```

## Documentation

Build the API documentation locally from a clone of the repository:
```sh
$ mise run docs:rust
```
