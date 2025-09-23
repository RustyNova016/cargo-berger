pub mod init;
use clap::Parser;
use clap::Subcommand;

use crate::cli::rust::workspace::init::RustWorkspaceInitCommand;

#[derive(Parser, Debug, Clone)]
pub struct RustWorkspaceCommand {
    #[command(subcommand)]
    pub command: RustWorkspaceSubcommand,
}

#[derive(Subcommand, Debug, Clone)]
pub enum RustWorkspaceSubcommand {
    Init(RustWorkspaceInitCommand),
}

impl RustWorkspaceCommand {
    pub fn run(&self) -> crate::ColEyre {
        match &self.command {
            RustWorkspaceSubcommand::Init(val) => val.run(),
        }
    }
}
