use std::process::Command;

use crate::models::ext::command::ExitStatusExt as _;
use crate::ColEyre;
use crate::models::shell_commands::shell::Shell;

pub struct GitCliff {
    shell: Shell,
}

impl GitCliff {
    pub fn new(shell: Shell) -> Self {
        Self { shell }
    }

    pub fn new_command(&self) -> Command {
        self.shell.new_command("git-cliff")
    }

    pub fn generate_changelog(&self) -> ColEyre {
        Ok(self
            .new_command()
            .arg("-o")
            .arg("CHANGELOG.md")
            .status()?
            .exit_on_non_zero())
    }
}
