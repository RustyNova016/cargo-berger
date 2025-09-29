use crate::ColEyre;
use crate::ColEyreVal;
use crate::infoln;
use crate::models::repository_data::RepositoryData;

impl RepositoryData {
    pub fn commit_checkpoint(&self, message: Option<&str>) -> ColEyre {
        self.remove_previous_tmps()?;

        if !self.repository.is_dirty()? {
            infoln!(
                "Repository `{}` is clean. Skipping checkpoint commit",
                self.name
            )
        }

        self.repository.add_all_files();
        self.repository.commit(&format!(
            "checkpoint: {}",
            message.unwrap_or("(No message)")
        ))?;

        Ok(())
    }

    pub fn is_latest_commit_checkpoint(&self) -> ColEyreVal<bool> {
        Ok(self
            .repository
            .get_latest_commit_name()?
            .starts_with("checkpoint:"))
    }
}
