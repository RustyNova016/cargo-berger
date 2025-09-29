use color_eyre::eyre::Context as _;

use crate::ColEyre;
use crate::models::tool_bindings::cargo::Cargo;
use crate::utils::cmd::assert_status;

impl Cargo {
    /// Runs `cargo semver-checks --all-features`
    pub fn cargo_semver(&self) -> ColEyre {
        assert_status(
            self.get_base_cargo_command()
                .arg("semver-checks")
                .arg("--all-features")
                .status()?,
        )
        .context("Error while verifying semmantic versioning")
    }
}
