use color_eyre::eyre::Context as _;

use crate::ColEyre;
use crate::models::tool_bindings::cargo::Cargo;
use crate::utils::cmd::assert_status;

impl Cargo {
    pub fn cargo_min_version(&self) -> ColEyre {
        assert_status(
            self.get_base_cargo_command()
                .arg("+nightly")
                .arg("minimal-versions")
                .arg("check")
                .status()?,
        )
        .context("Found dependancies with wrong minimal versions")
    }
}
