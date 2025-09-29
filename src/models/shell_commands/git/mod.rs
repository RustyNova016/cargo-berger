use std::process::Command;

use crate::models::shell_commands::shell::Shell;

pub mod add;
pub mod commit;
pub mod tag;



pub struct Git {
    shell: Shell,
}

impl Git {
    pub fn new(shell: Shell) -> Self {
        Self { shell }
    }

    pub fn new_command(&self) -> Command {
        self.shell.new_command("git")
    }
}


