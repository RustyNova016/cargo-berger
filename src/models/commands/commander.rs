use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;

use command_run::Command as CommandR;

use crate::models::commands::gh::GH;
use crate::models::commands::git::GitCLI;
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

    pub fn build_command<S: AsRef<OsStr>>(&self, program: S) -> CommandR {
        CommandR::new(program)
    }

    pub fn gh(self) -> GH {
        GH::new(self)
    }

    pub fn git(self) -> GitCLI {
        GitCLI::new(self)
    }

    pub fn git_cliff(self) -> GitCliff {
        GitCliff::new(self)
    }
}
