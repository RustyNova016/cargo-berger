use clap::Parser;

use crate::ColEyre;
use crate::infoln;
use crate::models::cli_data::CLI_DATA;

/// Save the changes and move to another branch.
#[derive(Parser, Debug, Clone)]
pub struct QuickSwitchCommand {
    /// The name of the branch
    branch: String,

    /// The message for the save commit
    message: Option<String>,
}

impl QuickSwitchCommand {
    pub fn run(&self) -> ColEyre {
        let berger = CLI_DATA.write().unwrap().get_berger_data()?;

        for repo_data in berger.repo_data.values() {
            infoln!("Processing repository `{}`", repo_data.name);

            repo_data.quick_switch_branch(&self.branch, None)?;
        }

        Ok(())
    }
}
