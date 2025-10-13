# Repository

| Name             | Required | Format                             | Default  | Description                                                                    |
| ---------------- | -------- | ---------------------------------- | -------- | ------------------------------------------------------------------------------ |
| path             | Yes      | String                             | -        | The path to the repository                                                     |
| inherit          | No       | Boolean                            | true     | If the repository contains a `berger.toml` file, use those settings as a basis |
| berger_file_path | No       | String                             | null     | Use this file as a basis for settings                                          |
| remote_url       | No       | String                             | null     | The url of the repository                                                      |
| default_remote   | No       | String                             | "origin" | The name of the remote repository                                              |
| default_branch   | No       | String                             | "master" | The default branch of the repository                                           |
| release          | No       | [Release Config](./release/mod.md) | Default  | Configuration for the releases                                                 |
| rust             | No       | [Rust Config](./rust/mod.md)       | null     | The rust configuration                                                         |

