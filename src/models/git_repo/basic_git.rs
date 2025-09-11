use std::io;
use std::io::Write as _;
use std::process::Command;

use git2::Status;

use crate::ColEyre;
use crate::ColEyreVal;
use crate::models::git_repo::GitRepo;

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
}
