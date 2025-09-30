use std::process::Command;

use crate::ColEyre;
use crate::models::commands::commander::Commander;
use crate::models::ext::exit_status::ExitStatusExt as _;

pub struct GitCliff {
    shell: Commander,
}

impl GitCliff {
    pub fn new(shell: Commander) -> Self {
        Self { shell }
    }

    pub fn new_command(&self) -> Command {
        self.shell.new_command("git-cliff")
    }

    pub fn generate_changelog(&self, version: &str) -> ColEyre {
        self.new_command()
            .arg("-o")
            .arg("CHANGELOG.md")
            .arg("--tag")
            .arg(version)
            .status()?
            .exit_on_non_zero();

        Ok(())
    }
}
