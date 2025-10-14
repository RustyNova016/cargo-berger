use std::process::exit;

use crate::errorln;
use crate::models::berger_data::BergerData;
use crate::models::repository_data::RepositoryData;

pub struct UserParser;

impl UserParser {
    pub fn parse_issue_repo<'b>(
        berger: &'b BergerData,
        issue_number: u64,
        repo_name: Option<&str>,
    ) -> &'b RepositoryData {
        if let Some(repo_name) = repo_name {
            if let Some(repo) = berger.repo_data.get(repo_name) {
                return repo;
            }

            errorln!("Repository `{repo_name}` isn't part of the workspace");
            exit(1);
        }

        if let Some(repo) = berger.find_repo_for_issue(issue_number) {
            return repo;
        }

        errorln!(
            "The repository from which the issue originate is ambiguous. Please precise the --repo argument"
        );
        exit(1);
    }
}
