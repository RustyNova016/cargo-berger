pub mod push;
use std::process::Command;

use crate::ColEyre;
use crate::models::commands::commander::Commander;
use crate::models::ext::io::output::OutputExt as _;

pub mod branch;

pub struct GitCLI {
    command: Commander,
}

impl GitCLI {
    pub fn new(command: Commander) -> Self {
        Self { command }
    }

    pub fn new_command(&self) -> Command {
        self.command.new_command("git")
    }

    pub fn build_command(&self) -> command_run::Command {
        self.command.build_command("git")
    }

    pub fn rev_parse_head(&self) -> ColEyre<String> {
        Ok(self
            .new_command()
            .arg("rev-parse")
            .arg("HEAD")
            .output()?
            .into_string())
    }
}
