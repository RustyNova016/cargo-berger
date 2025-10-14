use crate::ColEyre;
use crate::models::tool_bindings::git::GitRepo;
use crate::utils::cmd::assert_status;

impl GitRepo {
    pub fn rebase(&self, target: &str, interactive: bool) -> ColEyre {
        let mut command = self.get_base_command();
        command.arg("rebase").arg(target);

        if interactive {
            command.arg("-i");
        }

        assert_status(command.status()?)
    }

    pub fn rebase_on_remote_default(&self, interactive: bool) -> ColEyre {
        self.rebase(&format!("origin/{}", self.default_branch), interactive)
    }
}
