use clap::Parser;

use crate::ColEyre;
use crate::models::config::WorkplaceConfig;
use crate::models::crate_data::CrateData;

/// Makes a new temporary commit, which will be removed on next commit. Useful if you need to quickly save your work and switch to another branch
#[derive(Parser, Debug, Clone)]
pub struct QuickSwitchCommand {
    branch: String,
    message: Option<String>,
}

impl QuickSwitchCommand {
    pub fn run(&self) -> ColEyre {
        let mut conf = WorkplaceConfig::load()?;
        let crate_conf = conf.crates.pop().unwrap();
        let crate_data = CrateData::open_repo(crate_conf)?;

        crate_data.make_tmp_save_commit(
            self.message
                .as_deref()
                .or(Some(&format!("Quick switch to branch `{}`", &self.branch))),
        )?;

        crate_data
            .repository
            .switch_branch_or_create(&self.branch)?;

        Ok(())
    }
}
