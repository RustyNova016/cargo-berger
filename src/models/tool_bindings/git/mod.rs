use std::path::PathBuf;

use git2::Repository;

use crate::models::commands::commander::Commander;

pub mod basic_git;
pub mod branch;
pub mod clone;
pub mod rebase;
pub mod tag;

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

    pub fn new_command(&self) -> Commander {
        Commander::new(self.path.to_owned())
    }
}
