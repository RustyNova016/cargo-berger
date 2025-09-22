use std::path::Path;
use std::path::PathBuf;

use crate::ColEyreVal;
use crate::models::config::repository_config::RepositoryConfig;
use crate::models::repository_data::rust::RustData;
use crate::models::tool_bindings::git::GitRepo;

pub mod git;
pub mod rust;

/// The data of a single Berger repo
pub struct RepositoryData {
    pub name: String,

    /// Cannon path to this repo
    pub path: PathBuf,

    pub conf: RepositoryConfig,
    pub repository: GitRepo,

    pub rust: Option<RustData>,
}

impl RepositoryData {
    pub fn get_directory(&self) -> &Path {
        &self.path
    }

    pub fn open_repo(name: String, conf: RepositoryConfig) -> ColEyreVal<Self> {
        let path = PathBuf::from(conf.path.clone()).canonicalize()?;

        let repository = GitRepo::new(path.clone(), conf.default_branch.clone());

        let rust = conf
            .rust
            .clone()
            .map(|conf| RustData::load(&path, conf))
            .transpose()?;

        Ok(Self {
            name,
            path,
            conf,
            repository,

            rust,
        })
    }
}
