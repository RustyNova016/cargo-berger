use color_eyre::owo_colors::OwoColorize;

use crate::ColEyre;
use crate::infoln;
use crate::models::cli_data::CLI_DATA;

/// Amend the previous commit with everything in the current dir tree. This automatically add all the files
#[derive(clap::Parser, Debug, Clone)]
pub struct AmendCommand {
    /// Rewrite the commit message to this
    message: Option<String>,

    /// Push the changes using --force-with-lease
    #[clap(short, long)]
    push: bool,
}

impl AmendCommand {
    pub fn run(&self) -> ColEyre {
        let berger = CLI_DATA.write().unwrap().get_berger_data()?;

        for repo_data in berger.repo_data.values() {
            infoln!("Processing repository `{}`", repo_data.name);

            repo_data.repository.add_all_files();

            repo_data
                .repository
                .commit_ammend(self.message.as_deref())?;

            repo_data.repository.push(true, true, false)?;
        }

        infoln!();
        infoln!("{}", "✓ Successfully amended the berger workplace".green());

        Ok(())
    }
}
