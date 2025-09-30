# Commits

Cargo berger offers "types" of commits to allow easier management of the advancement of a feature

## TMP / Saves

Save commits are temporary commits. They don't hold any guaranty. They are used to clean the git tree, allowing for some commands to be run more safely, switching to other branches, etc...

Usually, only one save commit will exist, as the two other commit types ammend it. (It's actually a `git reset HEAD~` but looks like an amend)

## Checkpoints

Checkpoint commits are a step up from the save commits. While they are kept, they still don't hold any guaranty about the state of the code (it might not even compile!). This is useful to have a point in time to revert to if a change isn't going your way, and you need to restart with a different idea.

## Full

Full commits are commits that are "clean". This means the code works, has no lints, anyone can pull and run, and could be merged into `master` without fuss. 
Those commits should have conventional commit notation, although it's not enforced


## Commands

Those types of commits are translated to those commands:

- `cargo berger save [MESSAGE]`
- `cargo berger checkpoint [MESSAGE]`
- `cargo berger full <MESSAGE>`
