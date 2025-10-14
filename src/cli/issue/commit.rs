use clap::Parser;

use crate::infoln;
use crate::models::cli_data::CLI_DATA;

/// Initialise the folder using the data from the berger file. This needs to be rerun every major changes in the `berger.toml` file
#[derive(Parser, Debug, Clone)]
pub struct IssueCommitCommand;

impl IssueCommitCommand {
    pub fn run(&self) -> crate::ColEyre {
        let berger = CLI_DATA.write().unwrap().get_berger_data()?;

        for repo_data in berger.repo_data.values() {
            infoln!("Processing repository `{}`", repo_data.name);

            let current_branch_name = repo_data.repository.get_current_branch()?;
            let repo_issue = 
        }

        Ok(())
    }
}
