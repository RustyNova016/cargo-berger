# Commits

Cargo berger offers "types" of commits to allow easier management of the advancement a feature

## "saves"

Save commits are temporary commits. They don't hold any guaranty. They are used to clean the git tree, allowing for some commands to be run more safely, switching to other branches, etc...

## Checkpoints

Checkpoint commits are a step up from the save commits. While they are kept, they still don't hold any guaranty about the state of the code (it might not even compile!). This is useful to have a point in time to revert to if a change isn't going your way, and you need to restart with a diferent idea.

## Full

Full commits are commits that are "clean". This means the code works, has no lints, anyone can pull and run, and could be merged into `master` without fuss. 
Those commits should have conventional commit notation, although it's not enforced


- `cargo berger save`: This make a temporary commit, allowing pushing temporary changes or switching to an new branch.
- `cargo berger checkpoint`: This make a checkpoint commit, allowing pushing temporary changes or switching to an new branch.
