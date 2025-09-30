use convert_case::Casing;

use crate::ColEyre;
use crate::infoln;
use crate::models::repository_data::RepositoryData;

impl RepositoryData {
    pub fn pre_release(&self, version: &str) -> ColEyre {
        if let Some(rust) = &self.rust {
            rust.pre_release(version)?;
        }

        Ok(())
    }

    pub fn release(&self, version: &str) -> ColEyre {
        let branch_name = format!("release_{}", version.to_case(convert_case::Case::Snake));
        self.new_feat_branch(
            &branch_name,
            Some(&format!("Switch to release branch `{branch_name}`")),
        )?;
        self.commit_tmp(Some(&format!("Pre release `{}`", version)))?;

        infoln!("Preparing release...");
        self.pre_release(version)?;

        self.commit_full(&format!("chore(release): New version `{version}`"))?;
        self.repository
            .add_tag(version, &format!("New version: `{version}`"))?;

        Ok(())
    }
}
