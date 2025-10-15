pub mod checkout;

use clap::Parser;
use clap::Subcommand;

use crate::cli::issue::checkout::IssueCheckoutCommand;

#[derive(Parser, Debug, Clone)]
pub struct IssueCommand {
    #[command(subcommand)]
    pub command: IssueSubcommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum IssueSubcommands {
    Checkout(IssueCheckoutCommand),
}

impl IssueCommand {
    pub fn run(&self) -> crate::ColEyre {
        match &self.command {
            IssueSubcommands::Checkout(val) => val.run(),
        }
    }
}
