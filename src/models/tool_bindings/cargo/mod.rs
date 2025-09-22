pub mod cargo_file;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;

use color_eyre::eyre::Context;
use color_eyre::eyre::eyre;

use crate::ColEyre;
use crate::utils::cmd::assert_status;

pub struct Cargo {
    pub current_dir: PathBuf,
}

impl Cargo {
    pub fn new(current_dir: PathBuf) -> Self {
        Self { current_dir }
    }

    pub fn get_base_cargo_command(&self) -> Command {
        let mut cmd = Command::new("cargo");
        cmd.current_dir(&self.current_dir);
        cmd
    }

    pub fn fmt(&self) -> ColEyre {
        assert_status(self.get_base_cargo_command().arg("fmt").status()?)
            .context("Error while formating rust code")
    }

    pub fn clippy_hack(&self) -> ColEyre {
        let mut cmd = self.get_base_cargo_command();
        cmd.arg("hack")
            .arg("clippy")
            .arg("--feature-powerset")
            .arg("--")
            .arg("-D")
            .arg("warnings")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .env("SQLX_OFFLINE", "true");

        let output = cmd.output()?;
        if output.status.success() {
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(eyre!("Error while validating rust code: {}", error))
        }
    }

    pub fn sqlx_prepare(&self) -> ColEyre {
        let mut cmd = self.get_base_cargo_command();
        cmd.arg("sqlx")
            .arg("prepare")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        let output = cmd.output()?;
        if output.status.success() {
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(eyre!("Error while validating rust code: {}", error))
        }
    }
}
