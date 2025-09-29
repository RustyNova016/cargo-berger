use std::path::Path;
use std::path::PathBuf;

use color_eyre::eyre::Context;
use color_eyre::eyre::ContextCompat;
use color_eyre::eyre::eyre;

use crate::ColEyreVal;
use crate::models::commands::commander::Commander;
use crate::models::config::repository_config::RepositoryConfig;
use crate::models::repository_data::rust::RustData;
use crate::models::tool_bindings::git::GitRepo;

/// All the commit types and their associated functions
pub mod ci;
pub mod commits;
pub mod git;
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
    pub fn load(name: String, conf: RepositoryConfig) -> ColEyreVal<Self> {
        let path = PathBuf::from(conf.path.clone())
            .canonicalize()
            .context(eyre!("Couldn't find folder: `{}`", conf.path.clone()))?;

        let repository = GitRepo::new(path.clone(), conf.default_branch.clone());

        let rust = conf
            .rust
            .clone()
            .map(|rust_conf| RustData::load(&path, rust_conf, conf.ci.rust_ci.to_owned()))
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
    pub fn initialize_repo(name: String, conf: RepositoryConfig, root: &Path) -> ColEyreVal<Self> {
        // Check if don't have the repo cloned yet
        if !PathBuf::from(conf.path.clone()).exists() {
            let url = conf.remote_url.clone().context(eyre!("[`{name}`] Couldn't clone repository: remote_url is missing in the configuration file"))?;
            GitRepo::clone(root, &url, Some(&name))?
        }

        Self::load(name, conf)
    }

    pub fn get_directory(&self) -> &Path {
        &self.root_folder
    }

    pub fn new_command(&self) -> Commander {
        Commander::new(self.root_folder.to_owned())
    }
}
