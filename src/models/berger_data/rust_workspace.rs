use std::path::PathBuf;

use crate::ColEyre;
use crate::ColEyreVal;
use crate::models::berger_data::BergerData;
use crate::models::tool_bindings::cargo::cargo_file::CargoFile;

pub struct RustWorkspace {
    /// The root of the workspace, with the Cargo.toml file
    #[expect(dead_code)]
    workspace_root: PathBuf,
    cargo_file: CargoFile,
}

impl RustWorkspace {
    /// Load or create a workspace in the designated folder
    pub fn load_or_create(workspace_root: PathBuf) -> ColEyreVal<Self> {
        Ok(Self {
            cargo_file: CargoFile::load_or_create(workspace_root.join("Cargo.toml"))?,
            workspace_root,
        })
    }

    /// Initialize the workspace by adding all the members and patches
    pub fn init(&self, berger: &BergerData) -> ColEyre {
        self.cargo_file.set_workspace_resolver("3");

        for repo in berger.repo_data.values() {
            if let Some(_rust) = &repo.rust {
                self.cargo_file.add_crate_to_workspace(&repo.path)?;
                self.cargo_file
                    .add_local_patch(repo.name.to_string(), &repo.path);
            }
        }

        self.cargo_file.save()?;

        Ok(())
    }
}
