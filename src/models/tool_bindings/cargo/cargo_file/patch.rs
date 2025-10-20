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

    pub fn read_patches(&self) -> Vec<CargoPatch> {
        let doc = self.doc.lock().unwrap();

        let mut patches = Vec::new();
        for (crat, patch) in doc["patch"]["crates-io"]
            .as_table()
            .unwrap_or(&Default::default())
        {
            let Some(data) = patch.as_table() else {
                continue;
            };

            patches.push(CargoPatch {
                name: crat.to_owned(),
                path: data["path"].as_str().map(|s| s.to_string()),
                git: data["git"].as_str().map(|s| s.to_string()),
                branch: data["branch"].as_str().map(|s| s.to_string()),
                rev: data["rev"].as_str().map(|s| s.to_string()),
            });
        }

        patches
    }

    pub fn add_patch(&self, patch: CargoPatch) {
        let mut doc = self.doc.lock().unwrap();

        let patched = &mut Self::get_patch_crate_io_mut(&mut doc)[&patch.name];
        add_optional_element_to_item(patched, "path", patch.path);
        add_optional_element_to_item(patched, "git", patch.git);
        add_optional_element_to_item(patched, "branch", patch.branch);
        add_optional_element_to_item(patched, "rev", patch.rev);
    }

    pub fn remove_patches(&self) {
        let mut doc = self.doc.lock().unwrap();

        doc.remove("patch");
    }
}

pub struct CargoPatch {
    pub name: String,
    pub path: Option<String>,
    pub git: Option<String>,
    pub branch: Option<String>,
    pub rev: Option<String>,
}

pub fn add_optional_element_to_item<T>(item: &mut Item, key: &str, ele: Option<T>)
where
    Item: From<T>,
{
    if let Some(ele) = ele {
        item[key] = ele.into()
    }
}
