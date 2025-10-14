use crate::ColEyre;
use crate::models::commands::gh::issue::IssueInfo;
use crate::models::repository_data::RepositoryData;

impl RepositoryData {
    pub fn get_issue_info(&self, issue_number: u64) -> ColEyre<IssueInfo> {
        self.new_command().gh().issue_view(
            issue_number,
            self.conf.remote_url.as_ref().unwrap_or_else(|| {
                panic!("No `remote_url` has been configured for `{}`", self.name)
            }),
        )
    }
}
