use clap::Parser;

use crate::models::cli_data::CLI_DATA;

/// Pull the branch from the remote
#[derive(Parser, Debug, Clone)]
pub struct RustWorkspaceInitCommand;

impl RustWorkspaceInitCommand {
    pub fn run(&self) -> crate::ColEyre {
        let berger = CLI_DATA.write().unwrap().get_berger_data()?;

        berger
            .get_rust_workspace(true)?
            .expect("The workspace file should have been created")
            .init(&berger)?;

        Ok(())
    }
}
