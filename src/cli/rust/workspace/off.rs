use clap::Parser;
use itertools::Itertools as _;

use crate::models::cli_data::CLI_DATA;

/// Turn the workplace off. This doesn't actually delete any files, and only rename the cargo files to not be recognized by cargo.
///
/// This is useful to temporarly deactivate the workplace to run a command
///
/// You can turn it back with the `on` command
#[derive(Parser, Debug, Clone)]
pub struct RustWorkspaceOffCommand;

impl RustWorkspaceOffCommand {
    pub fn run(&self) -> crate::ColEyre {
        let berger = CLI_DATA.write().unwrap().get_berger_data()?;

        let repos = berger.repo_data.values().collect_vec();

        match berger.get_rust_workspace(true)? {
            Some(wp) => wp.turn_off(repos.clone())?,
            None => println!(
                "Couldn't turn off the workspace. No suitable `Cargo.toml` file has been found"
            ),
        }

        Ok(())
    }
}
