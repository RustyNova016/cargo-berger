use crate::ColEyre;
use crate::infoln;
use crate::models::repository_data::RepositoryData;

impl RepositoryData {
    pub fn quick_switch_branch(&self, branch: &str, tmp_message: Option<&str>) -> ColEyre {
        infoln!("Quick switch to branch `{branch}`");
        self.commit_tmp(tmp_message.or(Some(&format!("Quick switch to branch `{branch}`"))))?;

        self.repository.switch_branch(branch)?;
        Ok(())
    }

    pub fn quick_switch_to_default(&self, tmp_message: Option<&str>) -> ColEyre {
        self.quick_switch_branch(&self.conf.default_branch, tmp_message)
    }
}
