# Humble beginings

Cargo-berger is configuration lite, but not configuration less. To get started, you just have to create a folder with a `berger.toml` file inside

You can then use this minimal template to add a git repository

```toml
[repository.<name>]
path="./<name>"
```

... Yes. That's it. Berger will recognize your folder. However it's not that useful. You can then add the remote directory for the repository, and it will get automatically cloned when running a berger command:

```toml
[repository.<name>]
path="./<name>"
remote_url = <remote url>
```

You may also switch the default branch if you fancy something else:

```toml
[repository.<name>]
path="./<name>"
remote_url = <remote url>
default_branch = "king"
```

## "I don't have time to config" mode

If no `berger.toml` is found, cargo-berger will run with this configuration:

```toml
[repository.current]
path="./"
```

You'll still be able to run some commands, but others may require a config file