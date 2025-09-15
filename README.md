# cargo-berger

Cargo Berger is a cli app to organise multiple rust crates with different directories. This allow to orchestrate all git repositories as they were only one, without the cons of monorepos. 

While it is made for rust, multiple commands can be used on any git repository.

# Features:
- Multicrate commits using `save`, `checkpoint`, `full`.
- Create feature branches for all crates using `new-feat`
- Switch branches on all repos using `quick-switch`
- `pull`, `push`, and rebase onto `origin/master` with `refresh-default` 