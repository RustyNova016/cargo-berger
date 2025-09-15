use std::path::PathBuf;

use crate::ColEyreVal;
use crate::models::config::repository_config::RepositoryConfig;
use crate::models::tool_bindings::git::GitRepo;

pub mod git;
pub mod rust;

pub struct RepositoryData {
    pub conf: RepositoryConfig,
    pub repository: GitRepo,
}

impl RepositoryData {
    pub fn get_directory(&self) -> ColEyreVal<PathBuf> {
        Ok(PathBuf::from(&self.conf.path).canonicalize()?)
    }

    pub fn open_repo(conf: RepositoryConfig) -> ColEyreVal<Self> {
        let repository = GitRepo::new(
            PathBuf::from(conf.path.clone()).canonicalize()?,
            conf.default_branch.clone(),
        );
        Ok(Self { conf, repository })
    }
}
