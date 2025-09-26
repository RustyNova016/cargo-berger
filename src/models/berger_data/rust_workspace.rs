use std::fs::remove_dir_all;
use std::fs::rename;
use std::path::PathBuf;

use crate::ColEyre;
use crate::ColEyreVal;
use crate::models::berger_data::BergerData;
use crate::models::repository_data::RepositoryData;
use crate::models::tool_bindings::cargo::cargo_file::CargoFile;

pub struct RustWorkspace {
    /// The root of the workspace, with the Cargo.toml file
    workspace_root: PathBuf,
    cargo_file: CargoFile,
}

impl RustWorkspace {
    /// Load a workspace in the designated folder, and cancel if it doesn't exists
    pub fn try_load(workspace_root: PathBuf) -> ColEyreVal<Option<Self>> {
        let Some(cargo_file) = CargoFile::try_load(workspace_root.join("Cargo.toml"))? else {
            return Ok(None);
        };

        Ok(Some(Self {
            cargo_file,
            workspace_root,
        }))
    }

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
                self.cargo_file.add_crate_to_workspace(&repo.root_folder)?;
                self.cargo_file
                    .add_local_patch(repo.name.to_string(), &repo.root_folder);
            }
        }

        self.cargo_file.save()?;

        Ok(())
    }

    pub fn turn_off(&self) -> ColEyre {
        let cargo_file = self.workspace_root.join("Cargo.toml");
        if cargo_file.exists() {
            rename(cargo_file, self.workspace_root.join("Cargo.disabled.toml"))?;
        }

        let cargo_lock = self.workspace_root.join("Cargo.lock");
        if cargo_lock.exists() {
            rename(cargo_lock, self.workspace_root.join("Cargo.disabled.lock"))?;
        }

        let target_dir = self.workspace_root.join("target");
        if target_dir.exists() {
            rename(target_dir, self.workspace_root.join("target.disabled"))?;
        }

        Ok(())
    }

    pub fn turn_on(&self, repos: &[&RepositoryData]) -> ColEyre {
        let target_dir = self.workspace_root.join("target.disabled");
        if target_dir.exists() {
            rename(target_dir, self.workspace_root.join("target"))?;
        }

        let cargo_lock = self.workspace_root.join("Cargo.disabled.lock");
        if cargo_lock.exists() {
            rename(cargo_lock, self.workspace_root.join("Cargo.lock"))?;
        }

        let cargo_file = self.workspace_root.join("Cargo.disabled.toml");
        if cargo_file.exists() {
            rename(cargo_file, self.workspace_root.join("Cargo.toml"))?;
        }

        // Remove the target directories from the inner crates

        for repo in repos {
            // Only do rust repos
            if repo.rust.is_none() {
                continue;
            }

            let target = repo.root_folder.join("target");
            if target.exists() {
                remove_dir_all(target)?;
            }
        }

        Ok(())
    }
}
