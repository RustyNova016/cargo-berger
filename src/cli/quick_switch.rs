use clap::Parser;

use crate::ColEyre;
use crate::models::config::WorkplaceConfig;
use crate::models::crate_data::CrateData;

/// Save the changes and move to another branch.
#[derive(Parser, Debug, Clone)]
pub struct QuickSwitchCommand {
    /// The name of the branch
    branch: String,

    /// The message for the save commit
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

        crate_data.repository.switch_branch(&self.branch)?;

        Ok(())
    }
}
