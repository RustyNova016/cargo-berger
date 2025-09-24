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

        berger
            .get_rust_workspace(true)?
            .expect("The workspace file should have been created")
            .turn_on(&repos)?;

        Ok(())
    }
}
