use crate::models::ext::regex::RegexExt;
use crate::models::repository_data::RepositoryData;
use crate::utils::regex::repository_url::REPO_URL_REGEX;

pub mod pull_request;

impl RepositoryData {
    /// Get the owner of the remote
    pub fn get_remote_owner(&self) -> Option<&str> {
        let remote_url = self.conf.remote_url.as_ref()?;

        REPO_URL_REGEX
            .capture_first(remote_url)
            .and_then(|cap| cap.name("username"))
            .map(|mat| mat.as_str())
    }
}
