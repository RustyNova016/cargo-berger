use std::process::Command;

use crate::models::commands::commander::Commander;

pub mod branch;

pub struct Git {
    command: Commander,
}

impl Git {
    pub fn new(command: Commander) -> Self {
        Self { command }
    }

    pub fn new_command(&self) -> Command {
        self.command.new_command("git")
    }
}
