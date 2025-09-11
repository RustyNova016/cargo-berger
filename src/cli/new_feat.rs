use clap::Parser;

use crate::ColEyre;
use crate::models::cli_data::CLI_DATA;

/// Create a new feature branch, based on the default branch. This also pull the default branch for new changes, so the new branch is on par with latest
#[derive(Parser, Debug, Clone)]
pub struct NewFeatCommand {
    branch: String,
    message: Option<String>,
}

impl NewFeatCommand {
    pub fn run(&self) -> ColEyre {
        let crates = CLI_DATA.write().unwrap().get_crates_data()?;

        for crate_data in crates {
            println!("[ Processing Crate `{}`]", crate_data.conf.name);

            crate_data.make_tmp_save_commit(self.message.as_deref().or(Some(&format!(
                "Switch to new feature branch `{}`",
                &self.branch
            ))))?;

            crate_data.switch_to_default()?;

            crate_data
                .repository
                .switch_branch_or_create(&self.branch)?;
        }

        Ok(())
    }
}
