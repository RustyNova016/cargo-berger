use std::fs::remove_dir_all;
use std::fs::rename;
use std::path::PathBuf;

use itertools::Itertools;

use crate::ColEyre;
use crate::ColEyreVal;
use crate::models::berger_data::BergerData;
use crate::models::repository_data::RepositoryData;
use crate::models::tool_bindings::cargo::cargo_file::CargoFile;

pub mod patching;
pub struct RustWorkspace {
    /// The root of the workspace, with the Cargo.toml file
    workspace_root: PathBuf,
    cargo_file: CargoFile,
}

impl RustWorkspace {
    /// Load a workspace in the designated folder.
    ///
    /// Return [`None`] if the file doesn't exists and shouldn't be created, or exist but has a crate definition as well
    pub fn load(workspace_root: PathBuf, create: bool) -> ColEyreVal<Option<Self>> {
        let file_path = workspace_root.join("Cargo.toml");

        let cargo_file = if create {
            CargoFile::load_or_create(file_path)?
        } else {
            let Some(cargo_file) = CargoFile::try_load(file_path)? else {
                return Ok(None);
            };
            cargo_file
        };

        if cargo_file.has_crate_def() {
            return Ok(None);
        }

        Ok(Some(Self {
            cargo_file,
            workspace_root,
        }))
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

    pub fn turn_off(&self, repos: Vec<&RepositoryData>) -> ColEyre {
        let crate_repos = repos
            .into_iter()
            .filter(|repo| repo.rust.is_some())
            .collect_vec();

        self.install_patches_of_inner(&crate_repos)?;

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

    pub fn turn_on(&self, repos: Vec<&RepositoryData>) -> ColEyre {
        let crate_repos = repos
            .into_iter()
            .filter(|repo| repo.rust.is_some())
            .collect_vec();

        self.install_patch(&crate_repos)?;
        self.remove_patches_of_inner(&crate_repos)?;

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

        for repo in crate_repos {
            let target = repo.root_folder.join("target");
            if target.exists() {
                remove_dir_all(target)?;
            }
        }

        Ok(())
    }
}
