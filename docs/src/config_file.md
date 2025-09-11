# Config file

By default, cargo Berger check for a configuration file `berger.toml` in the current directory. This configuration file allows configuring the crates in the workplace.

## Exemple:

````toml
[[crates]]
name="cargo-berger"
path="./"
default_branch="master"

[[crates]]
name="shaun"
path="../shaun/"
default_branch="master"
```