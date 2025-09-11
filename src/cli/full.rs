use clap::Parser;

use crate::ColEyre;
use crate::models::config::WorkplaceConfig;
use crate::models::crate_data::CrateData;

/// Makes a new full commit. This commit is means a task is completed, the code is working and clean.
/// This type of commit is "ready to merge"
#[derive(Parser, Debug, Clone)]
pub struct FullCommand {
    message: String,
}

impl FullCommand {
    pub fn run(&self) -> ColEyre {
        let mut conf = WorkplaceConfig::load()?;
        let crate_conf = conf.crates.pop().unwrap();
        let crate_data = CrateData::open_repo(crate_conf)?;

        crate_data.make_full_commit(&self.message)?;

        Ok(())
    }
}
