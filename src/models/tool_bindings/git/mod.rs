pub mod clone;
use std::path::PathBuf;

use git2::Repository;

use crate::ColEyreVal;

pub mod basic_git;
pub mod commit;
pub mod rebase;

pub struct GitRepo {
    path: PathBuf,
    repo: Repository,
    default_branch: String,
}

impl GitRepo {
    pub fn new(path: PathBuf, default_branch: String) -> Self {
        Self {
            path: path.clone(),
            repo: Repository::open(path.canonicalize().unwrap()).unwrap(),
            default_branch,
        }
    }

    pub fn is_latest_commit_save(&self) -> ColEyreVal<bool> {
        Ok(self.get_latest_commit_name()?.starts_with("tmp:"))
    }
}
