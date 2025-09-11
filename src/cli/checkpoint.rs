use clap::Parser;

use crate::ColEyre;
use crate::models::cli_data::CLI_DATA;

/// Makes a new checkpoint commit. This commit is a savepoint in the development, and will stay in the tree.
/// However it doesn't make any guaranty on the quality of the code.
#[derive(Parser, Debug, Clone)]
pub struct CheckpointCommand {
    message: Option<String>,
}

impl CheckpointCommand {
    pub fn run(&self) -> ColEyre {
        let crates = CLI_DATA.write().unwrap().get_crates_data()?;

        for crate_data in crates {
            println!("[ Processing Crate `{}`]", crate_data.conf.name);

            crate_data
                .repository
                .make_checkpoint_commit(self.message.as_deref())?;
        }

        Ok(())
    }
}
