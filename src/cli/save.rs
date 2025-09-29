use clap::Parser;

use crate::ColEyre;
use crate::models::cli_data::CLI_DATA;

/// Makes a new temporary commit, which will be removed on next commit. Useful if you need to quickly save your work and switch to another branch
#[derive(Parser, Debug, Clone)]
pub struct SaveCommand {
    message: Option<String>,
}

impl SaveCommand {
    pub fn run(&self) -> ColEyre {
        let berger = CLI_DATA.write().unwrap().get_berger_data()?;

        for repo_data in berger.repo_data.values() {
            println!("[ Processing repository `{}`]", repo_data.name);

            repo_data.commit_tmp(self.message.as_deref())?;
        }

        Ok(())
    }
}
