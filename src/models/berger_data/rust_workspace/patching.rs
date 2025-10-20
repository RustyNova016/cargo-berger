use crate::ColEyre;
use crate::models::berger_data::rust_workspace::RustWorkspace;
use crate::models::repository_data::RepositoryData;
use crate::models::tool_bindings::cargo::cargo_file::patch::CargoPatch;

impl RustWorkspace {
    /// Add the patches for the workspace
    pub fn install_patch(&self, repos: &[&RepositoryData]) -> ColEyre {
        for repo in repos {
            if repo.rust.is_none() {
                continue;
            }

            let patch = CargoPatch {
                name: repo.name.to_owned(),
                path: Some(repo.root_folder.to_string_lossy().to_string()),
                branch: None,
                git: None,
                rev: None,
            };

            self.cargo_file.add_patch(patch);
            self.cargo_file.save()?;
        }

        Ok(())
    }

    pub fn remove_patches_of_inner(&self, repos: &[&RepositoryData]) -> ColEyre {
        for repo in repos {
            if let Some(rust) = &repo.rust {
                rust.remove_patches()?;
            }
        }

        Ok(())
    }

    pub fn install_patches_of_inner(&self, crates: &[&RepositoryData]) -> ColEyre {
        for repo in crates {
            if let Some(rust) = &repo.rust {
                rust.install_git_patches(crates)?;
            }
        }

        Ok(())
    }
}
