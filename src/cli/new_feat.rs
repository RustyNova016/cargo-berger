use clap::Parser;

use crate::ColEyre;
use crate::infoln;
use crate::models::cli_data::CLI_DATA;

/// Create a new feature branch, based on the default branch. This also pull the default branch for new changes, so the new branch is on par with latest
#[derive(Parser, Debug, Clone)]
pub struct NewFeatCommand {
    branch: String,
    message: Option<String>,
}

impl NewFeatCommand {
    pub fn run(&self) -> ColEyre {
        let berger = CLI_DATA.write().unwrap().get_berger_data()?;

        for repo_data in berger.repo_data.values() {
            infoln!("Processing repository `{}`", repo_data.name);

            repo_data.commit_tmp(self.message.as_deref().or(Some(&format!(
                "Switch to new feature branch `{}`",
                &self.branch
            ))))?;

            repo_data.switch_to_default()?;

            repo_data.repository.switch_branch_or_create(&self.branch)?;
        }

        Ok(())
    }
}
