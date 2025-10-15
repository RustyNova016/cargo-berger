use clap::Parser;

use crate::infoln;
use crate::models::cli_data::CLI_DATA;
use crate::utils::user_parsing::UserParser;

/// Initialise the folder using the data from the berger file. This needs to be rerun every major changes in the `berger.toml` file
#[derive(Parser, Debug, Clone)]
pub struct IssueCheckoutCommand {
    /// The issue number that this commit fixes
    issue_number: u64,

    /// Which repo this issue is from
    #[clap(short, long)]
    repo: Option<String>,
}

impl IssueCheckoutCommand {
    pub fn run(&self) -> crate::ColEyre {
        let berger = CLI_DATA.write().unwrap().get_berger_data()?;

        let issue_repo =
            UserParser::parse_issue_repo(berger.as_ref(), self.issue_number, self.repo.as_deref());
        let branch_name = format!("{}/issue_{}", issue_repo.name, self.issue_number);

        for repo_data in berger.repo_data.values() {
            infoln!("Processing repository `{}`", repo_data.name);

            repo_data.new_feat_branch(
                &branch_name,
                Some(&format!("Switch to issue branch `{}`", &branch_name)),
            )?;
        }

        Ok(())
    }
}
