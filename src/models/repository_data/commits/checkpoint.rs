use crate::ColEyre;
use crate::models::repository_data::RepositoryData;

impl RepositoryData {
    pub fn commit_checkpoint(&self, message: Option<&str>) -> ColEyre {
        self.remove_previous_tmps()?;

        if !self.repository.is_dirty()? {
            println!("[ Repository `{}` is clean. Skipping commit]", self.name)
        }

        self.repository.add_all_files();
        self.repository.commit(&format!(
            "checkpoint: {}",
            message.unwrap_or("(No message)")
        ))?;

        Ok(())
    }
}
