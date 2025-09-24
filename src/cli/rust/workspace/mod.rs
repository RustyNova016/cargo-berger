pub mod init;
pub mod off;
pub mod on;
use clap::Parser;
use clap::Subcommand;

use crate::cli::rust::workspace::init::RustWorkspaceInitCommand;
use crate::cli::rust::workspace::off::RustWorkspaceOffCommand;
use crate::cli::rust::workspace::on::RustWorkspaceOnCommand;

#[derive(Parser, Debug, Clone)]
pub struct RustWorkspaceCommand {
    #[command(subcommand)]
    pub command: RustWorkspaceSubcommand,
}

#[derive(Subcommand, Debug, Clone)]
pub enum RustWorkspaceSubcommand {
    Init(RustWorkspaceInitCommand),
    Off(RustWorkspaceOffCommand),
    On(RustWorkspaceOnCommand),
}

impl RustWorkspaceCommand {
    pub fn run(&self) -> crate::ColEyre {
        match &self.command {
            RustWorkspaceSubcommand::Init(val) => val.run(),
            RustWorkspaceSubcommand::On(val) => val.run(),
            RustWorkspaceSubcommand::Off(val) => val.run(),
        }
    }
}
