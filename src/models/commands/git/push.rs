use bon::bon;

use crate::ColEyre;
use crate::models::commands::git::GitCLI;
use crate::utils::cmd::error_on_non_zero;

#[bon]
impl GitCLI {
    #[builder]
    pub fn push(
        &self,
        set_upstream: Option<(&str, &str)>,
        #[builder(default)] force_with_lease: bool,
        #[builder(default)] force: bool,
    ) -> ColEyre {
        let mut cmd = self.build_command();
        cmd.add_arg("push");

        if let Some((remote, branch)) = set_upstream {
            cmd.add_arg("--set-upstream")
                .add_arg(remote)
                .add_arg(branch);
        }

        if force {
            cmd.add_arg("--force");
        }

        if force_with_lease {
            cmd.add_arg("--force-with-lease");
        }

        error_on_non_zero(cmd.run()?)
    }
}
