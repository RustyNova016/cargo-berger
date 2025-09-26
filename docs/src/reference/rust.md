# Rust

| Name        | Required | Format | Default        | Description                                                              |
| ----------- | -------- | ------ | -------------- | ------------------------------------------------------------------------ |
| fmt         | false    | bool   | false          | Run `cargo fmt` on full commit                                           |
| sqlx        | false    | bool   | false          | Run `cargo sqlx prepare` on full commit                                  |
| clippy_hack | false    | bool   | false          | Run `cargo hack clippy --feature-powerset -- -d warnings` on full commit |
| ci          | false    | [RustCI](#ci) | Default Values | The configuration for ci checks                                          |



## CI


| Name      | Required | Format | Default | Description                                                  |
| --------- | -------- | ------ | ------- | ------------------------------------------------------------ |
| fmt       | false    | bool   | true    | Check that the code is formated                              |
| clippy    | false    | bool   | false   | Run `cargo clippy -- -d warnings`                            |
| msrv      | false    | bool   | false   | Run `cargo msrv verify` to check if the msrv is correct      |
| msrv_find | false    | bool   | true    | If `cargo msrv verify` fails, find the new msrv of the crate |
