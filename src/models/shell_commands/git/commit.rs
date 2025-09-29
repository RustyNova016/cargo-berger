use crate::ColEyre;
use crate::models::ext::command::ExitStatusExt;
use crate::models::shell_commands::git::Git;

impl Git {
    pub fn commit(&self, message: &str) -> ColEyre {
        Ok(self
            .new_command()
            .arg("commit")
            .arg("-m")
            .arg(message)
            .status()?
            .exit_on_non_zero())
    }
}
