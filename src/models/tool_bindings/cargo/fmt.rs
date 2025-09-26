use color_eyre::eyre::Context as _;

use crate::ColEyre;
use crate::models::tool_bindings::cargo::Cargo;
use crate::utils::cmd::assert_status;

impl Cargo {
    /// Runs `cargo fmt`
    pub fn fmt(&self) -> ColEyre {
        assert_status(self.get_base_cargo_command().arg("fmt").status()?)
            .context("Error while formating rust code")
    }

    /// Runs `cargo fmt --all --check` to check if the current project is formated
    pub fn fmt_check(&self) -> ColEyre {
        assert_status(
            self.get_base_cargo_command()
                .arg("fmt")
                .arg("--all")
                .arg("--check")
                .status()?,
        )
        .context("Error while formating rust code")
    }
}
