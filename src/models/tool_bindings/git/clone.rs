use std::path::Path;
use std::process::Command;

use crate::ColEyre;
use crate::models::tool_bindings::git::GitRepo;
use crate::utils::cmd::assert_status;

impl GitRepo {
    pub fn clone(base_dir: &Path, url: &str, name: Option<&str>) -> ColEyre {
        let mut cmd = Command::new("git");
        cmd.current_dir(base_dir).arg("clone").arg(url);

        if let Some(name) = name {
            cmd.arg(name);
        }

        assert_status(cmd.status()?)
    }
}
