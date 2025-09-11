use clap::Parser;

use crate::ColEyre;
use crate::models::cli_data::CLI_DATA;

/// Save the changes and move to another branch.
#[derive(Parser, Debug, Clone)]
pub struct PRCommand {}

impl PRCommand {
    pub fn run(&self) -> ColEyre {
        let crates = CLI_DATA.write().unwrap().get_crates_data()?;

        for crate_data in crates {
            println!("[ Processing Crate `{}`]", crate_data.conf.name);

            crate_data.make_pr()?;
        }

        Ok(())
    }
}
