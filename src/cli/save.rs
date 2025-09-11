use clap::Parser;

use crate::ColEyre;
use crate::models::cli_data::CLI_DATA;

/// Makes a new temporary commit, which will be removed on next commit. Useful if you need to quickly save your work and switch to another branch
#[derive(Parser, Debug, Clone)]
pub struct SaveCommand {
    message: Option<String>,
}

impl SaveCommand {
    pub fn run(&self) -> ColEyre {
        let crates = CLI_DATA.write().unwrap().get_crates_data()?;

        for crate_data in crates {
            println!("[ Processing Crate `{}`]", crate_data.conf.name);

            crate_data
                .repository
                .make_tmp_save_commit(self.message.as_deref())?;
        }

        Ok(())
    }
}
