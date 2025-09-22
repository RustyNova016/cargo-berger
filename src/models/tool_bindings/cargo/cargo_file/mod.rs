pub mod workplace;
use std::fs::File;
use std::io::Read as _;
use std::path::Path;

use color_eyre::eyre::Context as _;
use toml_edit::DocumentMut;

use crate::ColEyreVal;

pub struct CargoFile {
    doc: DocumentMut,
}

impl CargoFile {
    pub fn load(path: &Path) -> ColEyreVal<Self> {
        let mut file = File::open(path)
            .context("Couldn't open the berger config file. Make sure it exists")?;
        let mut data = String::new();
        file.read_to_string(&mut data)
            .context("Couldn't read the autosort config file")?;

        Ok(Self { doc: data.parse()? })
    }

    pub fn add_git_patch(&mut self, dep_name: String, git: String, rev: String) {
        self.doc["patch"]["crates-io"][dep_name.clone()]["git"] = git.into();
        self.doc["patch"]["crates-io"][dep_name]["rev"] = rev.into();
    }

    pub fn add_local_patch(&mut self, dep_name: String, path: &Path) {
        self.doc["patch"]["crates-io"][dep_name.clone()]["path"] =
            path.to_string_lossy().to_string().into();
    }
}
