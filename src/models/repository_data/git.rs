use std::process::Command;

use color_eyre::eyre::Ok;

use crate::ColEyre;
use crate::models::repository_data::RepositoryData;
use crate::models::tool_bindings::git::commit::CommitResult;

impl RepositoryData {
    pub fn make_tmp_save_commit(&self, message: Option<&str>) -> ColEyre {
        if let CommitResult::CleanTree = self.repository.make_tmp_save_commit(message)? {
            println!("[ Repository `{}` is clean. Skipping commit]", self.name)
        };

        Ok(())
    }

    pub fn switch_to_default(&self) -> ColEyre {
        self.repository.switch_branch(&self.conf.default_branch)?;
        self.repository.pull_branch()
    }

    pub fn make_pr(&self) -> ColEyre {
        if self.repository.is_branch_empty(&self.conf.default_branch)? {
            println!(
                "[ Skipping PR for repository `{}` (No changes) ]",
                self.name
            );

            return Ok(());
        }

        Command::new("gh")
            .arg("pr")
            .arg("create")
            .current_dir(&self.conf.path)
            .status()?;

        Ok(())
    }
}
