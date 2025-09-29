use std::path::PathBuf;
use std::process::Command;

pub struct Shell {
    current_dir: PathBuf,
}

impl Shell {
    pub fn new(current_dir: PathBuf) -> Self {
        Self { current_dir }
    }

    pub fn new_command(&self, cmd: &str) -> Command {
        let mut cmd = Command::new(cmd);
        cmd.current_dir(&self.current_dir);
        cmd
    }
}
