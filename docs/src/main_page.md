# `cargo-berger`

Cargo Berger is a cli app to organise multiple rust crates with different directories. This allow to orchestrate all git repositories as they were only one, without the cons of monorepos. 

While it is made for rust, multiple commands can be used on any git repository.

This app assume a github-flow workflow. This means:
- Creating a branch from `origin/master`
- Creating commits on this "feature branch"
- Creating a PR with the changes onto master

Please note that while this documentation assumes the remote repository is `origin`, and the default branch is `master`, they can be configured.

Use the sidebar to get started