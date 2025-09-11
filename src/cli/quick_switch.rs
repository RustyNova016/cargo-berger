use clap::Parser;

use crate::ColEyre;
use crate::models::cli_data::CLI_DATA;

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
        let crates = CLI_DATA.write().unwrap().get_crates_data()?;

        for crate_data in crates {
            println!("[ Processing Crate `{}`]", crate_data.conf.name);

            crate_data.make_tmp_save_commit(
                self.message
                    .as_deref()
                    .or(Some(&format!("Quick switch to branch `{}`", &self.branch))),
            )?;

            crate_data.repository.switch_branch(&self.branch)?;
        }

        Ok(())
    }
}
