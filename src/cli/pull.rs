use clap::Parser;

use crate::ColEyre;
use crate::models::cli_data::CLI_DATA;

/// Pull the branch from the remote
#[derive(Parser, Debug, Clone)]
pub struct PullCommand;

impl PullCommand {
    pub fn run(&self) -> ColEyre {
        let berger = CLI_DATA.write().unwrap().get_berger_data()?;

        for repo_data in berger.repo_data.values() {
            println!("[ Processing repository `{}`]", repo_data.name);

            repo_data.repository.fetch(None, None)?;
            repo_data.repository.pull_branch()?;
        }

        Ok(())
    }
}
