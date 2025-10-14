use crate::models::berger_data::BergerData;
use crate::models::repository_data::RepositoryData;

impl BergerData {
    pub fn find_repo_for_issue(&self, _issue_number: u64) -> Option<&RepositoryData> {
        // If there's only one repo, it's this one
        if self.repo_data.len() == 1 {
            return self.repo_data.values().next();
        }

        None
    }
}
