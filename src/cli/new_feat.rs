use clap::Parser;

use crate::ColEyre;
use crate::models::config::WorkplaceConfig;
use crate::models::crate_data::CrateData;

/// Create a new feature branch, based on the default branch. This also pull the default branch for new changes, so the new branch is on par with latest
#[derive(Parser, Debug, Clone)]
pub struct NewFeatCommand {
    branch: String,
    message: Option<String>,
}

impl NewFeatCommand {
    pub fn run(&self) -> ColEyre {
        let mut conf = WorkplaceConfig::load()?;
        let crate_conf = conf.crates.pop().unwrap();
        let crate_data = CrateData::open_repo(crate_conf)?;

        crate_data.make_tmp_save_commit(self.message.as_deref().or(Some(&format!(
            "Switch to new feature branch `{}`",
            &self.branch
        ))))?;

        crate_data.switch_to_default()?;

        crate_data
            .repository
            .switch_branch_or_create(&self.branch)?;

        Ok(())
    }
}
