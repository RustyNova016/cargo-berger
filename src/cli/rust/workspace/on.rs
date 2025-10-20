use clap::Parser;
use itertools::Itertools;

use crate::models::cli_data::CLI_DATA;

/// Toggle the workplace back on, removing the inner target directories of the crates
#[derive(Parser, Debug, Clone)]
pub struct RustWorkspaceOnCommand;

impl RustWorkspaceOnCommand {
    pub fn run(&self) -> crate::ColEyre {
        let berger = CLI_DATA.write().unwrap().get_berger_data()?;

        let repos = berger.repo_data.values().collect_vec();

        match berger.get_rust_workspace(true)? {
            Some(wp) => wp.turn_on(repos.clone())?,
            None => println!(
                "Couldn't turn on the workspace. No suitable `Cargo.toml` file has been found"
            ),
        }

        Ok(())
    }
}
