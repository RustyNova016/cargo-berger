use clap::Parser;

use crate::ColEyre;
use crate::models::cli_data::CLI_DATA;

/// Makes a new full commit. This commit is means a task is completed, the code is working and clean.
/// This type of commit is "ready to merge"
#[derive(Parser, Debug, Clone)]
pub struct FullCommand {
    message: String,
}

impl FullCommand {
    pub fn run(&self) -> ColEyre {
        let berger = CLI_DATA.write().unwrap().get_berger_data()?;

        for repo_data in berger.repo_data.values() {
            println!("[ Processing repository `{}`]", repo_data.name);

            repo_data.commit_full(&self.message)?;
        }

        Ok(())
    }
}
