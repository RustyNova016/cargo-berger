use crate::models::tool_bindings::cargo::cargo_file::CargoFile;

impl CargoFile {
    pub fn get_package_name(&self) -> String {
        self.doc.lock().unwrap()["package"]["name"]
            .as_str()
            .unwrap()
            .to_string()
    }

    pub fn set_package_version(&self, version: &str) {
        let mut doc = self.doc.lock().unwrap();
        doc["package"]["version"] = version.into()
    }
}
