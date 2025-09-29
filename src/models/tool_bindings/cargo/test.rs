use color_eyre::eyre::Context as _;

use crate::ColEyre;
use crate::models::tool_bindings::cargo::Cargo;
use crate::utils::cmd::assert_status;

impl Cargo {
    /// Runs `cargo test`
    pub fn test(&self) -> ColEyre {
        assert_status(
            self.get_base_cargo_command()
                .arg("test")
                .arg("--all-features")
                .status()?,
        )
        .context("Error while formating rust code")
    }

    /// Runs `cargo test`
    pub fn nextest_run(&self) -> ColEyre {
        assert_status(
            self.get_base_cargo_command()
                .arg("nextest")
                .arg("run")
                .arg("--all-features")
                .status()?,
        )
        .context("Error while formating rust code")
    }
}
