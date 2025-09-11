pub mod commit;
use std::path::PathBuf;

use git2::Repository;

use crate::ColEyreVal;

pub mod basic_git;

pub struct GitRepo {
    path: PathBuf,
    repo: Repository,
}

impl GitRepo {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path: path.clone(),
            repo: Repository::open(path.canonicalize().unwrap()).unwrap(),
        }
    }

    pub fn is_latest_commit_save(&self) -> ColEyreVal<bool> {
        Ok(self.get_latest_commit_name()?.starts_with("tmp:"))
    }
}
