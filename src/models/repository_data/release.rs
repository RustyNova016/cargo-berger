use crate::models::repository_data::RepositoryData;
use crate::models::shell_commands::git_cliff::GitCliff;
use crate::ColEyre;

impl RepositoryData {
    /// Prepare everything for release
    pub fn pre_release(&self) {
        
    }

    pub fn release(&self, version: &str) -> ColEyre {
        self.make_tmp_save_commit(Some(&format!("Pre release `{}`", version)));
        let git = self.get_git_shell();

        if self.conf.release.git_cliff {
            GitCliff::new(self.get_shell()).generate_changelog()?;
        }

        self.remove_previous_tmps()?;
        git.add_all()?;
        git.commit(&format!("chore(release): New version `{}`", version))?;
        git.add_tag(version)?;

        Ok(())
    }
}