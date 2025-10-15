use itertools::Itertools;

use crate::ColEyre;
use crate::models::commands::git::Git;
use crate::models::ext::io::output::OutputExt;

impl Git {
    pub fn branch_list(&self) -> ColEyre<Vec<String>> {
        let branches = self
            .new_command()
            .arg("branch")
            .arg("--list")
            .output()?
            .into_string();

        Ok(branches.split("\n").map(|s| s.to_string()).collect_vec())
    }
}
