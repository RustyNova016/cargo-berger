use crate::ColEyreVal;
use crate::errorln;
use crate::models::repository_data::RepositoryData;

impl RepositoryData {
    pub fn run_ci(&self) -> ColEyreVal<bool> {
        if self.conf.ci.full_commit {
            println!("\n === Running full commit check ===\n");
            let name = self.repository.get_latest_commit_name()?;
            println!("{}", name);
            if !self.is_latest_commit_full()? {
                errorln!(
                    "CI Error: The latest commit isn't a full commit. Please run `cargo berger full <MESSAGE>` to set it as a full commit"
                );
                return Ok(false);
            }
        }

        if let Some(rust) = &self.rust {
            rust.run_ci()?;
        }

        Ok(true)
    }
}
