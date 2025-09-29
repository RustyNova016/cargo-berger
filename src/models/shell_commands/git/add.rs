use crate::ColEyre;
use crate::models::ext::command::ExitStatusExt;
use crate::models::shell_commands::git::Git;

impl Git {
    /// Stage all the changes
    pub fn add_all(&self) -> ColEyre {
        Ok(self
            .new_command()
            .arg("add")
            .arg(".")
            .status()?
            .exit_on_non_zero())
    }
}
