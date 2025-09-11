use clap::Parser;

use crate::ColEyre;
use crate::models::cli_data::CLI_DATA;

/// Makes a new full commit. This commit is means a task is completed, the code is working and clean.
/// This type of commit is "ready to merge"
#[derive(Parser, Debug, Clone)]
pub struct FullCommand {
    message: String,
}

impl FullCommand {
    pub fn run(&self) -> ColEyre {
        let crates = CLI_DATA.write().unwrap().get_crates_data()?;

        for crate_data in crates {
            println!("[ Processing Crate `{}`]", crate_data.conf.name);

            crate_data.make_full_commit(&self.message)?;
        }

        Ok(())
    }
}
