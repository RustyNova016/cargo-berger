use std::path::Path;
use std::path::PathBuf;

use color_eyre::eyre::Context;
use color_eyre::eyre::eyre;

use crate::ColEyreVal;
use crate::models::commands::commander::Commander;
use crate::models::config::repository_config::RepositoryConfig;
use crate::models::repository_data::rust::RustData;
use crate::models::tool_bindings::git::GitRepo;

/// All the commit types and their associated functions
pub mod branches;
pub mod commits;
pub mod git;
pub mod release;
pub mod remotes;
pub mod rust;

/// The data of a single Berger repo
pub struct RepositoryData {
    pub name: String,

    /// Cannon path to this repo
    pub root_folder: PathBuf,

    pub conf: RepositoryConfig,
    pub repository: GitRepo,

    // === Language Handlers ===
    pub rust: Option<RustData>,
}

impl RepositoryData {
    /// Load an existing repo
    pub fn load(name: String, conf: RepositoryConfig, wp_root: &Path) -> ColEyreVal<Self> {
        let path = conf.full_path(wp_root);

        if !path.exists() {
            return Err(eyre!("Couldn't find path: {}", path.display()));
        }

        let repository = GitRepo::new(path.clone(), conf.default_branch.clone());

        let rust = conf
            .rust
            .clone()
            .map(|conf| RustData::load(&path, conf))
            .transpose()?;

        Ok(Self {
            name,
            root_folder: path,
            conf,
            repository,

            rust,
        })
    }

    /// Initialize and load the repo. It won't do anything if the repository is already initialized,
    /// so it shouldn't break anything over just loading the repo
    pub fn initialize_repo(
        name: String,
        conf: RepositoryConfig,
        wp_root: &Path,
    ) -> ColEyreVal<Self> {
        // Check if don't have the repo cloned yet
        if !conf.full_path(wp_root).exists() {
            match conf.remote_url.clone() {
                Some(remote) => GitRepo::clone(wp_root, &remote, Some(&name))?,
                None => {
                    panic!(
                        "Error while initialising [`{name}`]:\n
                    Folder `{}` doesn't exists, but not `remote_url` has been configurated",
                        conf.full_path(wp_root).display()
                    )
                }
            }
        }

        Self::load(name, conf, wp_root)
    }

    pub fn get_directory(&self) -> &Path {
        &self.root_folder
    }

    pub fn open_repo(name: String, conf: RepositoryConfig) -> ColEyreVal<Self> {
        let path = PathBuf::from(conf.path.clone())
            .canonicalize()
            .context(eyre!("Couldn't find folder: `{}`", conf.path.clone()))?;

        let repository = GitRepo::new(path.clone(), conf.default_branch.clone());

        let rust = conf
            .rust
            .clone()
            .map(|conf| RustData::load(&path, conf))
            .transpose()?;

        Ok(Self {
            name,
            root_folder: path,
            conf,
            repository,

            rust,
        })
    }

    pub fn new_command(&self) -> Commander {
        Commander::new(self.root_folder.to_owned())
    }
}
