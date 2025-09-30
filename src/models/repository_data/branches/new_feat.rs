use crate::ColEyre;
use crate::models::repository_data::RepositoryData;

impl RepositoryData {
    pub fn new_feat_branch(&self, branch: &str, tmp_message: Option<&str>) -> ColEyre {
        self.quick_switch_to_default(
            tmp_message.or(Some(&format!("Switch to new feature branch `{branch}`"))),
        )?;

        self.repository.pull_branch()?;

        self.repository.switch_branch_or_create(branch)?;

        Ok(())
    }
}
