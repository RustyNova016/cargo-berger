use std::process::Command;

use color_eyre::eyre::Ok;

use crate::ColEyre;
use crate::models::crate_data::CrateData;
use crate::models::git_repo::commit::CommitResult;

impl CrateData {
    pub fn make_tmp_save_commit(&self, message: Option<&str>) -> ColEyre {
        if let CommitResult::CleanTree = self.repository.make_tmp_save_commit(message)? {
            println!("[ Crate `{}` is clean. Skipping commit]", self.conf.name)
        };

        Ok(())
    }

    pub fn make_checkpoint_commit(&self, message: Option<&str>) -> ColEyre {
        if let CommitResult::CleanTree = self.repository.make_checkpoint_commit(message)? {
            println!("[ Crate `{}` is clean. Skipping commit]", self.conf.name)
        };

        Ok(())
    }

    pub fn make_full_commit(&self, message: &str) -> ColEyre {
        if (!self.repository.is_dirty()?) && !self.repository.is_latest_commit_save()? {
            println!("[ Crate `{}` is clean. Skipping commit]", self.conf.name);
            return Ok(());
        }

        self.make_tmp_save_commit(Some(&format!("Before full commit `{}`", message)))?;

        println!("\n === Running Formater ===\n");
        self.fmt()?;

        if self.conf.sqlx {
            println!("\n === Running sqlx prepare ===\n");
            self.sqlx_prepare()?;
        }

        println!("\n === Running Clippy ===\n");
        self.clippy_hack()?;

        println!("\n === Creating commit ===\n");
        self.repository.make_full_commit(message)?;
        Ok(())
    }

    pub fn switch_to_default(&self) -> ColEyre {
        self.repository.switch_branch(&self.conf.default_branch)?;
        self.repository.pull_branch()
    }

    pub fn make_pr(&self) -> ColEyre {
        if self.repository.is_branch_empty(&self.conf.default_branch)? {
            println!(
                "[ Skipping PR for crate `{}` (No changes) ]",
                self.conf.name
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
