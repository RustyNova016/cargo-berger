use crate::ColEyre;
use crate::models::repository_data::RepositoryData;

impl RepositoryData {
    pub fn switch_to_default(&self) -> ColEyre {
        self.repository.switch_branch(&self.conf.default_branch)?;
        self.repository.pull_branch()
    }
}
