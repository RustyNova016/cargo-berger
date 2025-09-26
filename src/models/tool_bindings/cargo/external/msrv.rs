use color_eyre::eyre::Context as _;

use crate::ColEyre;
use crate::models::tool_bindings::cargo::Cargo;
use crate::utils::cmd::assert_status;

impl Cargo {
    /// Runs `cargo msrv verify`
    pub fn cargo_msrv_verify(&self) -> ColEyre {
        assert_status(
            self.get_base_cargo_command()
                .arg("msrv")
                .arg("verify")
                .status()?,
        )
        .context("Error while verifying the msrv")
    }

    /// Runs `cargo msrv find`
    pub fn cargo_msrv_find(&self) -> ColEyre {
        assert_status(
            self.get_base_cargo_command()
                .arg("msrv")
                .arg("find")
                .status()?,
        )
        .context("Error while find the msrv")
    }
}
