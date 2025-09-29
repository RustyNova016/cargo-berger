use crate::ColEyre;
use crate::models::ext::command::ExitStatusExt as _;
use crate::models::shell_commands::git::Git;

impl Git {
    /// Add a tag to current commit
    pub fn add_tag(&self, name: &str) -> ColEyre {
        Ok(self
            .new_command()
            .arg("-a")
            .arg(name)
            .status()?
            .exit_on_non_zero())
    }
}
