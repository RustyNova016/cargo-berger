use crate::models::repository_data::rust::RustData;
use crate::ColEyre;

impl RustData {
    pub fn pre_release(&self, version: &str) -> ColEyre {
        self.cargo_file.set_package_version(version);

        Ok(())
    }
}