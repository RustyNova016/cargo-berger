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
        let berger = CLI_DATA.write().unwrap().get_berger_data()?;

        for repo_data in berger.repo_data.values() {
            println!("[ Processing repository `{}`]", repo_data.name);

            repo_data
                .repository
                .make_checkpoint_commit(self.message.as_deref())?;
        }

        Ok(())
    }
}
