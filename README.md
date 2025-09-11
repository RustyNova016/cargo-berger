# cargo-berger

Cargo Berger is a cli app to organise multiple rust crates with different directories. This allow to orchestrate all git repositories as they were only one, without the cons of monorepos. 

# Features:
- Multicrate commits using `save`, `checkpoint`, `full`.
- Create feature branches for all crates using `new-feat`
- Switch branches on all repos using `quick-switch`