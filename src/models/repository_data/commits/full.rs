use crate::ColEyre;
use crate::ColEyreVal;
use crate::infoln;
use crate::models::repository_data::RepositoryData;

impl RepositoryData {
    pub fn commit_full(&self, message: &str) -> ColEyre {
        self.commit_tmp(Some(&format!("Before full commit `{}`", message)))?;

        self.pre_full_commit()?;

        self.remove_previous_tmps()?;

        // If there's nothing since last checkpoint, we overwrite the checkpoint
        if !self.repository.is_dirty()? && self.is_latest_commit_checkpoint()? {
            self.repository.reset_last_commit()?;
        }

        // If there's truly nothing, then skip commit
        if !self.repository.is_dirty()? {
            infoln!("Repository `{}` is clean. Skipping full commit", self.name)
        }

        self.repository.add_all_files();
        self.repository.commit(message)?;

        Ok(())
    }

    pub fn pre_full_commit(&self) -> ColEyre {
        if let Some(rust) = &self.rust {
            rust.pre_full_commit()?;
        }

        Ok(())
    }

    /// Return true if the lastest commit is a full commit
    pub fn is_latest_commit_full(&self) -> ColEyreVal<bool> {
        let name = self.repository.get_latest_commit_name()?;

        Ok(!name.starts_with("tmp:") && !name.starts_with("checkpoint:"))
    }
}
