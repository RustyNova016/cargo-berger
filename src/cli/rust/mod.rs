pub mod workspace;

use clap::Parser;
use clap::Subcommand;

use crate::cli::rust::workspace::RustWorkspaceCommand;

#[derive(Parser, Debug, Clone)]
pub struct RustCommand {
    #[command(subcommand)]
    pub command: RustSubcommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum RustSubcommands {
    Workspace(RustWorkspaceCommand),
}

impl RustCommand {
    pub fn run(&self) -> crate::ColEyre {
        match &self.command {
            RustSubcommands::Workspace(val) => val.run(),
        }
    }
}
