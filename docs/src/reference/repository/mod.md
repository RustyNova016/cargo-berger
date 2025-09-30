# Repository

| Name           | Required | Format                         | Default  | Description                          |
| -------------- | -------- | ------------------------------ | -------- | ------------------------------------ |
| path           | Yes      | String                         | -        | The path to the repository           |
| remote_url     | false    | String                         | null     | The url of the repository            |
| default_remote | false    | String                         | "origin" | The name of the remote repository    |
| default_branch | false    | String                         | "master" | The default branch of the repository |
| release        | false    | [Release Config](./release/mod.md) | Default  | Configuration for the releases       |
| rust           | false    | [Rust Config](./rust/mod.md)       | null     | The rust configuration               |
