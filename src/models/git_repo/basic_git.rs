use std::io;
use std::io::Write as _;
use std::process::Command;

use color_eyre::eyre::eyre;
use git2::Status;

use crate::ColEyre;
use crate::ColEyreVal;
use crate::models::git_repo::GitRepo;
use crate::utils::cmd::unwrap_status;

impl GitRepo {
    pub fn get_base_command(&self) -> Command {
        let mut cmd = Command::new("git");
        cmd.current_dir(&self.path);
        cmd
    }

    pub fn get_latest_commit_name(&self) -> ColEyreVal<String> {
        let mut cmd = Command::new("git");
        let cmd = cmd
            .arg("log")
            .arg("-1")
            .arg("--pretty=%B")
            .current_dir(&self.path);

        let output = cmd.output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn add_all_files(&self) {
        let mut cmd = Command::new("git");
        let cmd = cmd.arg("add").arg(".").current_dir(&self.path);

        let output = cmd.output().unwrap();
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }

    pub fn commit(&self, message: &str) -> ColEyre {
        let mut cmd = self.get_base_command();
        //format!("tmp: {}", message.unwrap_or("(No message)"))
        cmd.arg("commit").arg("-m").arg(message);

        let output = cmd.output().unwrap();
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        Ok(())
    }

    pub fn commit_ammend(&self, message: &str) -> ColEyre {
        let mut cmd = self.get_base_command();
        //format!("tmp: {}", message.unwrap_or("(No message)"))
        cmd.arg("commit").arg("-m").arg(message).arg("--amend");

        let output = cmd.output().unwrap();
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        Ok(())
    }

    pub fn is_dirty(&self) -> ColEyreVal<bool> {
        let statuses = self.repo.statuses(None)?;

        for status in statuses.iter() {
            let status = status.status();
            if !(status.is_ignored() || status == Status::CURRENT) {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn switch_branch(&self, branch: &str) -> ColEyre {
        unwrap_status(self.get_base_command().arg("switch").arg(branch).output()?)
    }

    pub fn create_and_switch_branch(&self, branch: &str) -> ColEyre {
        unwrap_status(
            self.get_base_command()
                .arg("switch")
                .arg("-c")
                .arg(branch)
                .output()?,
        )
    }

    pub fn switch_branch_or_create(&self, branch: &str) -> ColEyre {
        if self.branch_exists(branch)? {
            self.switch_branch(branch)
        } else {
            self.create_and_switch_branch(branch)
        }
    }

    pub fn pull_branch(&self) -> ColEyre {
        unwrap_status(self.get_base_command().arg("pull").output()?)
    }

    pub fn branch_exists(&self, branch: &str) -> ColEyreVal<bool> {
        let stat = self
            .get_base_command()
            .arg("show-ref")
            .arg("--quiet")
            .arg("--branches")
            .arg(branch)
            .status()?;

        if stat.code().unwrap() == 0 {
            Ok(true)
        } else if stat.code().unwrap() == 1 {
            Ok(false)
        } else {
            Err(eyre!(
                "Error status when check if branch `{branch}` exists: {}",
                stat.code().unwrap()
            ))
        }
    }

    pub fn reset_last_commit(&self) -> ColEyre {
        unwrap_status(self.get_base_command().arg("reset").arg("HEAD~").output()?)
    }
}
