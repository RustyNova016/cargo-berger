use std::path::PathBuf;

use git2::Repository;

pub mod basic_git;
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
}
