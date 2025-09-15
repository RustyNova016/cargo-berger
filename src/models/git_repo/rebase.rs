use crate::ColEyre;
use crate::models::git_repo::GitRepo;
use crate::utils::cmd::assert_status;

impl GitRepo {
    pub fn rebase(&self, target: &str) -> ColEyre {
        assert_status(self.get_base_command().arg("rebase").arg(target).status()?)
    }

    pub fn rebase_on_remote_default(&self) -> ColEyre {
        self.rebase(&format!("origin/{}", self.default_branch))
    }
}
