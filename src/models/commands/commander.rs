use std::path::PathBuf;
use std::process::Command;

use crate::models::commands::gh::GH;

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
}
