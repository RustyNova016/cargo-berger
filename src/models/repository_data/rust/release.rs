use crate::ColEyre;
use crate::models::repository_data::rust::RustData;

impl RustData {
    /// Pre release for rust
    pub fn pre_release(&self, version: &str) -> ColEyre {
        if self.cargo_file.has_crate_def() {
            self.cargo_file.set_package_version(version);
        }

        self.cargo_file.save()?;
        Ok(())
    }
}
