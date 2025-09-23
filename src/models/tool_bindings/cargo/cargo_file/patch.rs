use std::path::Path;

use toml_edit::DocumentMut;
use toml_edit::Item;
use toml_edit::Table;

use crate::models::tool_bindings::cargo::cargo_file::CargoFile;

impl CargoFile {
    pub fn get_patch_crate_io_mut(doc: &mut DocumentMut) -> &mut Item {
        doc.entry("patch")
            .or_insert_with(|| Item::Table(Table::new()))
            .as_table_like_mut()
            .expect("`patch` isn't a table")
            .entry("crates-io")
            .or_insert_with(|| Item::Table(Table::new()))
    }
    pub fn add_git_patch(&self, dep_name: String, git: String, rev: String) {
        let mut doc = self.doc.lock().unwrap();

        Self::get_patch_crate_io_mut(&mut doc)[dep_name.clone()]["git"] = git.into();
        Self::get_patch_crate_io_mut(&mut doc)[dep_name]["rev"] = rev.into();
    }

    pub fn add_local_patch(&self, dep_name: String, path: &Path) {
        let mut doc = self.doc.lock().unwrap();

        Self::get_patch_crate_io_mut(&mut doc)[dep_name.clone()]["path"] =
            path.to_string_lossy().to_string().into();
    }
}
