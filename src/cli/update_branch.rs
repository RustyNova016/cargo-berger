use clap::Parser;

use crate::ColEyre;
use crate::models::cli_data::CLI_DATA;

/// Rebase the current branch on the remote's default branch.
#[derive(Parser, Debug, Clone)]
pub struct RebaseDefaultCommand;

impl RebaseDefaultCommand {
    pub fn run(&self) -> ColEyre {
        let crates = CLI_DATA.write().unwrap().get_crates_data()?;

        for crate_data in crates {
            println!("[ Processing Crate `{}`]", crate_data.conf.name);

            crate_data.make_tmp_save_commit(Some("Rebasing branch on remote default"))?;
            crate_data.repository.fetch(None, None)?;
            crate_data.repository.rebase_on_remote_default()?;
        }

        Ok(())
    }
}
