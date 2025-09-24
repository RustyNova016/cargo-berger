# Rust


| Name          | Required | Format | Default | Description                                                              |
| ------------- | -------- | ------ | ------- | ------------------------------------------------------------------------ |
| fmt           | false    | bool   | false   | Run `cargo fmt` on full commit                                           |
| sqlx          | false    | bool   | false   | Run `cargo sqlx prepare` on full commit                                  |
| clippy_hack   | false    | bool   | false   | Run `cargo hack clippy --feature-powerset -- -d warnings` on full commit |
