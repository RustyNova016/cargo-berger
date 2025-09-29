use crate::ColEyre;
use crate::ColEyreVal;
use crate::models::repository_data::RepositoryData;

impl RepositoryData {
    pub fn commit_full(&self, message: &str) -> ColEyre {
        self.commit_tmp(Some(&format!("Before full commit `{}`", message)))?;

        self.pre_full_commit()?;

        self.remove_previous_tmps()?;

        if !self.repository.is_dirty()? {
            println!("[ Repository `{}` is clean. Skipping commit]", self.name)
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
