use crate::ColEyreVal;
use crate::models::tool_bindings::git::GitRepo;

pub enum CommitResult {
    /// Created a new commit
    Commited,

    /// Added to the last commit
    Amended,

    /// Remove the last edit and committed
    Replaced,

    /// The tree was clean so no changes where done
    CleanTree,
}

impl GitRepo {
    pub fn make_tmp_save_commit(&self, message: Option<&str>) -> ColEyreVal<CommitResult> {
        let mut out = None;
        if self.is_latest_commit_save()? {
            self.reset_last_commit()?;
            out.replace(CommitResult::Replaced);
        }

        if !self.is_dirty()? {
            return Ok(CommitResult::CleanTree);
        }

        self.add_all_files();

        self.commit(&format!("tmp: {}", message.unwrap_or("(No message)")))?;

        Ok(out.unwrap_or(CommitResult::Commited))
    }

    pub fn make_checkpoint_commit(&self, message: Option<&str>) -> ColEyreVal<CommitResult> {
        let mut out = None;
        if self.is_latest_commit_save()? {
            self.reset_last_commit()?;
            out.replace(CommitResult::Replaced);
        }

        if !self.is_dirty()? {
            return Ok(CommitResult::CleanTree);
        }

        self.add_all_files();

        self.commit(&format!(
            "checkpoint: {}",
            message.unwrap_or("(No message)")
        ))?;

        Ok(out.unwrap_or(CommitResult::Commited))
    }

    pub fn make_full_commit(&self, message: &str) -> ColEyreVal<CommitResult> {
        let mut out = None;
        if self.is_latest_commit_save()? {
            self.reset_last_commit()?;
            out.replace(CommitResult::Replaced);
        }

        if !self.is_dirty()? {
            return Ok(CommitResult::CleanTree);
        }

        self.add_all_files();

        self.commit(message)?;

        Ok(out.unwrap_or(CommitResult::Commited))
    }
}
