use serde::Deserialize;

use crate::ColEyre;
use crate::models::commands::gh::GH;
use crate::models::ext::io::output::OutputExt as _;

impl GH {
    pub fn issue_view(&self, issue_number: u64, repo: &str) -> ColEyre<IssueInfo> {
        self.new_command()
            .arg("issue")
            .arg("view")
            .arg(issue_number.to_string())
            .arg("--repo")
            .arg(repo)
            .arg("--json")
            .arg("title")
            .output()?
            .deserialize()
    }
}

#[derive(Debug, Deserialize)]
pub struct IssueInfo {
    pub title: String,
}
