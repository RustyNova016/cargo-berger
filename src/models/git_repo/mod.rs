use std::path::PathBuf;

use git2::Repository;

use crate::ColEyre;
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

    pub fn make_tmp_save_commit(&self, message: Option<&str>) -> ColEyre {
        self.add_all_files();

        if self.is_latest_commit_save()? {
            self.commit_ammend(&format!("tmp: {}", message.unwrap_or("(No message)")))
        } else {
            self.commit(&format!("tmp: {}", message.unwrap_or("(No message)")))
        }
    }

    pub fn make_checkpoint_commit(&self, message: Option<&str>) -> ColEyre {
        self.add_all_files();

        if self.is_latest_commit_save()? {
            self.commit_ammend(&format!(
                "checkpoint: {}",
                message.unwrap_or("(No message)")
            ))
        } else {
            self.commit(&format!(
                "checkpoint: {}",
                message.unwrap_or("(No message)")
            ))
        }
    }

    pub fn make_full_commit(&self, message: &str) -> ColEyre {
        self.add_all_files();

        if self.is_latest_commit_save()? {
            self.commit_ammend(message)
        } else {
            self.commit(message)
        }
    }
}
