use std::path::PathBuf;
use std::process::Command;

use crate::models::commands::gh::GH;
use crate::models::commands::git::Git;
use crate::models::commands::git_cliff::GitCliff;

/// Root for a command binding.
pub struct Commander {
    current_dir: PathBuf,
}

impl Commander {
    pub fn new(current_dir: PathBuf) -> Self {
        Self { current_dir }
    }

    pub fn new_command(&self, cmd: &str) -> Command {
        let mut cmd = Command::new(cmd);
        cmd.current_dir(&self.current_dir);
        cmd
    }

    pub fn gh(self) -> GH {
        GH::new(self)
    }

    pub fn git(self) -> Git {
        Git::new(self)
    }

    pub fn git_cliff(self) -> GitCliff {
        GitCliff::new(self)
    }
}
