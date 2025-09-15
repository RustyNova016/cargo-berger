use clap::Parser;

use crate::ColEyre;
use crate::models::cli_data::CLI_DATA;

/// Pull the branch from the remote
#[derive(Parser, Debug, Clone)]
pub struct PullCommand;

impl PullCommand {
    pub fn run(&self) -> ColEyre {
        let crates = CLI_DATA.write().unwrap().get_crates_data()?;

        for crate_data in crates {
            println!("[ Processing Crate `{}`]", crate_data.conf.name);

            crate_data.repository.fetch(None, None)?;
            crate_data.repository.pull_branch()?;
        }

        Ok(())
    }
}
