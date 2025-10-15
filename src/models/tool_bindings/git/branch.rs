use crate::ColEyre;
use crate::models::tool_bindings::git::GitRepo;

impl GitRepo {
    pub fn list_branches(&self) -> ColEyre<Vec<String>> {
        self.new_command().git().branch_list()
    }

    pub fn has_branch(&self, name: &str) -> ColEyre<bool> {
        Ok(self.list_branches()?.iter().any(|branch| branch == name))
    }
}
