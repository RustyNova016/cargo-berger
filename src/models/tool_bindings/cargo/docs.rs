use color_eyre::eyre::Context as _;

use crate::ColEyre;
use crate::models::tool_bindings::cargo::Cargo;
use crate::utils::cmd::assert_status;

impl Cargo {
    /// Runs `cargo doc` to check if the current project has correct docs
    pub fn doc_check(&self) -> ColEyre {
        assert_status(
            self.get_base_cargo_command()
                .arg("doc")
                .arg("--no-deps")
                .arg("--document-private-items")
                .arg("--all-features")
                .status()?,
        )
        .context("Error while formating rust code")
    }
}
