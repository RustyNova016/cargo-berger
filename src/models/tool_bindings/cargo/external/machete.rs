use color_eyre::eyre::Context as _;

use crate::ColEyre;
use crate::models::tool_bindings::cargo::Cargo;
use crate::utils::cmd::assert_status;

impl Cargo {
    /// Runs `cargo machete`
    pub fn cargo_machete(&self) -> ColEyre {
        assert_status(self.get_base_cargo_command().arg("machete").status()?)
            .context("Found useless dependancies")
    }
}
