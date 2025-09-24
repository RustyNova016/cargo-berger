# Getting started with Rust

All you need to do is add an object on the `rust` field of the repository object

```toml
[repository.cargo-berger]
path="./cargo-berger/"
default_branch="master"

rust={}
```

Now this repository will be recognized as a rust repository. You can then use `cargo-berger rust workspace init` to create a workspace with all the other rust crates in the config file.

You can also add some precommit checks:

```toml
[repository.cargo-berger]
path="./cargo-berger/"
default_branch="master"

    [repository.cargo-berger.rust]
    fmt = true # Run cargo fmt
    clippy_hack = true # Run cargo hack clippy
    sqlx = false # Run cargo sqlx prepare
```