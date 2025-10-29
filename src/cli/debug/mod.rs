use clap::Parser;
use clap::Subcommand;

use crate::cli::debug::list::DebugListCommand;

pub mod list;

#[derive(Parser, Debug, Clone)]
pub struct DebugCommand {
    #[command(subcommand)]
    pub command: DebugSubcommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum DebugSubcommands {
    List(DebugListCommand),
}

impl DebugCommand {
    pub fn run(&self) -> crate::ColEyre {
        match &self.command {
            DebugSubcommands::List(val) => val.run(),
        }
    }
}
