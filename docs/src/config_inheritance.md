# Config inheritance

Having one big config file at your workspace root is not always the most convenient. Lots of repositories makes it harder to find what you need, and you cannot use the configuration checked in your repository for the CI.

Luckily, it's actually pretty simple to split the file. Here's an example config: 

`./berger.toml`
```toml
[repository.cargo-berger]
path="./cargo-berger/"

[repository.shaun]
path="./shaun/"

default_branch="baaaaster"
```

Let's move `shaun` to its own file. For that, create a `berger.toml` file at `./shaun/berger.toml`, and copy the repository description. Back in your workspace's `berger.toml`, keep only the path to the repository

`./shaun/berger.toml`
```toml
[repository.shaun]
path="./shaun/"

default_branch="baaaaster"
```

`./berger.toml`
```toml
[repository.cargo-berger]
path="./cargo-berger/"

[repository.shaun]
path="./shaun/"
```

That's it! Now cargo-berger will take the configuration from `./shaun/berger.toml`. 

## Overwriting

Any setting set in the main `./berger.toml` will overwrite settings set in the child file

`./berger.toml`
```toml
...

[repository.shaun]
path="./shaun/"

default_branch="master" # "./shaun/berger.toml has a typo, PR is pending, in the meantime let's correct it here"
```

## Ignoring

Inheritance is activated by default. You can turn it off using: 

`./berger.toml`
```toml
...

[repository.shaun]
path="./shaun/"
inherit=false
```

## Find the config

You may also specify a path to the config file if it has a diferent path 

`./berger.toml`
```toml
...

[repository.shaun]
path="./shaun/"
berger_file_path="./berger.shaun.toml" # Use our own file
```
