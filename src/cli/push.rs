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
        let berger = CLI_DATA.write().unwrap().get_berger_data()?;

        for repo_data in berger.repo_data.values() {
            println!("[ Processing repository `{}`]", repo_data.name);

            repo_data.repository.push(true, self.lease, self.force)?;
        }

        Ok(())
    }
}
