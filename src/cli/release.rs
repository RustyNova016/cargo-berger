use clap::Parser;

use crate::ColEyre;
use crate::errorln;
use crate::models::cli_data::CLI_DATA;

/// Create a new release tag. Also do some checks before it
///
/// This command is not available as a batch workspace process. You need to specify the repo to use
#[derive(Parser, Debug, Clone)]
pub struct ReleaseCommand {
    /// The repository to make the release on
    repo: String,

    /// The new version of the repository
    version: String,
}

impl ReleaseCommand {
    pub fn run(&self) -> ColEyre {
        let berger = CLI_DATA.write().unwrap().get_berger_data()?;

        let Some(repo_data) = berger.repo_data.get(&self.repo) else {
            errorln!("Couldn't find repository `{}`", self.repo);
            return Ok(());
        };

        repo_data.release(&self.version)?;

        Ok(())
    }
}
