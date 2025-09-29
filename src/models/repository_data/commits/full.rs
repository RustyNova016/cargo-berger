use crate::ColEyre;
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
}
