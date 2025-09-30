use crate::models::tool_bindings::cargo::cargo_file::CargoFile;

impl CargoFile {
    pub fn set_package_version(&self, version: &str) {
        let mut doc = self.doc.lock().unwrap();
        doc["package"]["version"] = version.into()
    }
}
