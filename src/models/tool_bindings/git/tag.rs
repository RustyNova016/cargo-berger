use crate::ColEyre;
use crate::models::ext::exit_status::ExitStatusExt as _;
use crate::models::tool_bindings::git::GitRepo;

impl GitRepo {
    /// Add a tag to current commit
    pub fn add_tag(&self, name: &str, message: &str) -> ColEyre {
        self.get_base_command()
            .arg("tag")
            .arg("-a")
            .arg(name)
            .arg("-m")
            .arg(message)
            .status()?
            .exit_on_non_zero();
        Ok(())
    }
}
