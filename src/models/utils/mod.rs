use std::env;
use std::path::Path;
use std::process::Command;

pub fn run_in_dir(path: &Path, mut cmd: Command) -> Command {
    let current_dir = env::current_dir()
        .inspect_err(|error| {
            eprintln!("Could not determine current directory: {error}");
        })
        .unwrap();

    env::set_current_dir(path)
        .inspect_err(|error| eprintln!("Could not change directory to {path:?}: {error}"))
        .unwrap();

    cmd.env("WHENCE", &current_dir);

    cmd
}
