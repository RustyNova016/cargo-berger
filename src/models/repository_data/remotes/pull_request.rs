use crate::ColEyre;
use crate::infoln;
use crate::models::repository_data::RepositoryData;

impl RepositoryData {
    pub fn create_pull_request(&self) -> ColEyre {
        // We create PRs if the branch is not empty, and we'll not make a tmp commits
        if self.repository.is_branch_empty(&self.conf.default_branch)?
            && !self.repository.is_dirty()?
        {
            infoln!(
                "Repository `{}` has no commits. Skipping Pull Request",
                self.name
            );
        }

        self.commit_tmp(Some("Creating PR for branch"))?;

        self.new_command().gh().create_pr()?;

        Ok(())
    }
}
