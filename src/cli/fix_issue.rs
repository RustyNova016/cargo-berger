use std::process::exit;

use clap::Parser;

use crate::ColEyre;
use crate::errorln;
use crate::infoln;
use crate::models::cli_data::CLI_DATA;

/// Create a full commit for fixing a github issue
#[derive(Parser, Debug, Clone)]
pub struct FixIssueCommand {
    /// The issue number that this commit fixes
    issue_number: u64,

    /// Conventional commit prefix
    #[clap(default_value = "")]
    prefix: String,

    /// Which repo this issue is from
    #[clap(short, long)]
    repo: Option<String>,

    /// Push the data right after
    #[clap(long)]
    push: bool,

    /// Create a PR right after
    #[clap(short, long)]
    pr: bool,
}

impl FixIssueCommand {
    pub fn run(&self) -> ColEyre {
        let berger = CLI_DATA.write().unwrap().get_berger_data()?;

        //TODO: Smarter repo detection
        let repo = match self.repo.as_ref() {
            Some(repo) => berger.get_repository_or_exit(repo),
            None if berger.repo_data.len() == 1 => berger.repo_data.values().next().unwrap(),
            None => {
                errorln!(
                    "The repository from which the issue originate is ambiguous. Please precise the --repo argument"
                );
                exit(1);
            }
        };

        let info = repo.get_issue_info(self.issue_number)?;
        let commit_message = if self.prefix.trim().is_empty() {
            format!("feat: {}\n\nCloses: #{}", info.title, self.issue_number)
        } else {
            format!(
                "{}: {}\n\nCloses: #{}",
                self.prefix.trim(),
                info.title,
                self.issue_number
            )
        };

        for repo_data in berger.repo_data.values() {
            infoln!("Processing repository `{}`", repo_data.name);

            repo_data.commit_full(&commit_message)?;

            if self.push {
                repo_data.repository.push(true, false, false)?;
            }

            if self.pr {
                repo_data.create_pull_request()?;
            }
        }

        Ok(())
    }
}
