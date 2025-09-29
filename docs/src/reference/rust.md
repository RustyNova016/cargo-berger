# Rust

| Name        | Required | Format | Default        | Description                                                              |
| ----------- | -------- | ------ | -------------- | ------------------------------------------------------------------------ |
| fmt         | false    | bool   | false          | Run `cargo fmt` on full commit                                           |
| sqlx        | false    | bool   | false          | Run `cargo sqlx prepare` on full commit                                  |
| clippy_hack | false    | bool   | false          | Run `cargo hack clippy --feature-powerset -- -d warnings` on full commit |
| ci          | false    | [RustCI](#ci) | Default Values | The configuration for ci checks                                          |



## CI

Please note that not being on by default doesn't mean "not recommended"! Most of them requires additional binaries, which means extra attention


| Name         | Required | Format | Default | Description                                                  |
| ------------ | -------- | ------ | ------- | ------------------------------------------------------------ |
| fmt          | false    | bool   | true    | Check that the code is formated                              |
| clippy       | false    | bool   | true    | Run `cargo clippy -- -d warnings`                            |
| test         | false    | bool   | true    | Run tests                                                    |
| nextest      | false    | bool   | false   | Run tests with nextest (Needs test to be true)               |
| msrv         | false    | bool   | false   | Run `cargo msrv verify` to check if the msrv is correct      |
| msrv_find    | false    | bool   | true    | If `cargo msrv verify` fails, find the new msrv of the crate |
| machete      | false    | bool   | false   | Check for unused dependancies with `cargo-machete`            |
| semver       | false    | bool   | false   | Check for semver breakage using `cargo-semver`               |
| min-versions | false    | bool   | false   | Check if the minimum version of the dependancies are correct  |
