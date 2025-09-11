use clap::Parser;

use crate::ColEyre;
use crate::models::config::WorkplaceConfig;
use crate::models::crate_data::CrateData;

/// Makes a new checkpoint commit. This commit is a savepoint in the development, and will stay in the tree.
/// However it doesn't make any guaranty on the quality of the code.
#[derive(Parser, Debug, Clone)]
pub struct CheckpointCommand {
    message: Option<String>,
}

impl CheckpointCommand {
    pub fn run(&self) -> ColEyre {
        let mut conf = WorkplaceConfig::load()?;
        let crate_conf = conf.crates.pop().unwrap();
        let crate_data = CrateData::open_repo(crate_conf)?;

        crate_data
            .repository
            .make_checkpoint_commit(self.message.as_deref())?;

        Ok(())
    }
}
