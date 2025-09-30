use std::process::Stdio;

use crate::ColEyre;
use crate::errorln;
use crate::models::ext::exit_status::ExitStatusExt;
use crate::models::tool_bindings::cargo::Cargo;
use crate::utils::cmd::assert_status;

impl Cargo {
    /// Run cargo clippy
    pub fn clippy(&self) -> ColEyre {
        assert_status(
            self.get_base_cargo_command()
                .arg("clippy")
                .arg("--")
                .arg("-D")
                .arg("warnings")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .env("SQLX_OFFLINE", "true")
                .status()?,
        )
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
            .env("SQLX_OFFLINE", "true")
            .status()?
            .exit_on_non_zero_with(|| errorln!("Error while validating rust code"));

        Ok(())
    }
}
