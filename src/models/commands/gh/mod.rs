use std::process::Command;

use crate::ColEyre;
use crate::models::commands::commander::Commander;
use crate::models::ext::exit_status::ExitStatusExt as _;

pub struct GH {
    command: Commander,
}

impl GH {
    pub fn new(command: Commander) -> Self {
        Self { command }
    }

    pub fn new_command(&self) -> Command {
        self.command.new_command("gh")
    }

    pub fn create_pr(&self) -> ColEyre {
        self.new_command()
            .arg("pr")
            .arg("create")
            .status()?
            .exit_on_non_zero();

        Ok(())
    }
}
