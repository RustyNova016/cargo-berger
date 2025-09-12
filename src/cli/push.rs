use clap::Parser;

use crate::ColEyre;
use crate::models::cli_data::CLI_DATA;

/// Push the branch to the remote
#[derive(Parser, Debug, Clone)]
pub struct PushCommand {
    /// Force the push to the remote, but prevent overwriting commits from others
    #[clap(short, long)]
    lease: bool,

    /// Force the push to the remote
    #[clap(short, long)]
    force: bool,
}

impl PushCommand {
    pub fn run(&self) -> ColEyre {
        let crates = CLI_DATA.write().unwrap().get_crates_data()?;

        for crate_data in crates {
            println!("[ Processing Crate `{}`]", crate_data.conf.name);

            crate_data.repository.push(true, self.lease, self.force)?;
        }

        Ok(())
    }
}
