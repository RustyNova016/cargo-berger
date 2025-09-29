use clap::Parser;

use crate::ColEyre;
use crate::models::cli_data::CLI_DATA;

/// Rebase the current branch on the remote's default branch.
#[derive(Parser, Debug, Clone)]
pub struct RebaseDefaultCommand;

impl RebaseDefaultCommand {
    pub fn run(&self) -> ColEyre {
        let berger = CLI_DATA.write().unwrap().get_berger_data()?;

        for repo_data in berger.repo_data.values() {
            println!("[ Processing repository `{}`]", repo_data.name);

            repo_data.commit_tmp(Some("Rebasing branch on remote default"))?;
            repo_data.repository.fetch(None, None)?;
            repo_data.repository.rebase_on_remote_default()?;
        }

        Ok(())
    }
}
