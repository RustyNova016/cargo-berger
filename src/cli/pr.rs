use clap::Parser;

use crate::ColEyre;
use crate::infoln;
use crate::models::cli_data::CLI_DATA;

/// Create a pull request. Requires github cli to be installed
#[derive(Parser, Debug, Clone)]
pub struct PRCommand {}

impl PRCommand {
    pub fn run(&self) -> ColEyre {
        let berger = CLI_DATA.write().unwrap().get_berger_data()?;

        for repo_data in berger.repo_data.values() {
            infoln!("Processing repository `{}`", repo_data.name);

            repo_data.create_pull_request()?;
        }

        Ok(())
    }
}
