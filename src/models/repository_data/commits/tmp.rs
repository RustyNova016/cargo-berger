use crate::ColEyre;
use crate::ColEyreVal;
use crate::models::repository_data::RepositoryData;

impl RepositoryData {
    pub fn commit_tmp(&self, message: Option<&str>) -> ColEyre {
        self.remove_previous_tmps()?;

        if !self.repository.is_dirty()? {
            println!("[ Repository `{}` is clean. Skipping commit]", self.name)
        }

        self.repository.add_all_files();
        self.repository
            .commit(&format!("tmp: {}", message.unwrap_or("(No message)")))?;

        Ok(())
    }

    /// Remove all the previous temporary commits
    pub fn remove_previous_tmps(&self) -> ColEyre {
        while self.is_latest_commit_tmp()? {
            self.repository.reset_last_commit()?;
        }

        Ok(())
    }

    pub fn is_latest_commit_tmp(&self) -> ColEyreVal<bool> {
        Ok(self
            .repository
            .get_latest_commit_name()?
            .starts_with("tmp:"))
    }
}
