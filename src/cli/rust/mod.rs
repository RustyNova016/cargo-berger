pub mod ci;
pub mod workspace;

use clap::Parser;
use clap::Subcommand;

use crate::cli::rust::ci::RustCICommand;
use crate::cli::rust::workspace::RustWorkspaceCommand;

#[derive(Parser, Debug, Clone)]
pub struct RustCommand {
    #[command(subcommand)]
    pub command: RustSubcommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum RustSubcommands {
    CI(RustCICommand),
    Workspace(RustWorkspaceCommand),
}

impl RustCommand {
    pub fn run(&self) -> crate::ColEyre {
        match &self.command {
            RustSubcommands::CI(val) => val.run(),
            RustSubcommands::Workspace(val) => val.run(),
        }
    }
}
