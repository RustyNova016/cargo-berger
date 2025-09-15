pub mod cargo_bindings;
pub mod git;
use std::path::PathBuf;

use crate::ColEyreVal;
use crate::models::config::CrateConfig;
use crate::models::git_repo::GitRepo;

pub struct CrateData {
    pub conf: CrateConfig,
    pub repository: GitRepo,
}

impl CrateData {
    pub fn open_repo(conf: CrateConfig) -> ColEyreVal<Self> {
        // let repository = Repository::open(&conf.path)?;
        // Ok(Self {
        //     conf,
        //     repository
        // })

        let repository = GitRepo::new(
            PathBuf::from(conf.path.clone()),
            conf.default_branch.clone(),
        );
        Ok(Self { conf, repository })
    }

    // pub fn create_feature_branch(&self) -> ColEyre {
    //     let mut remote = self.repository.find_remote("origin")?;
    //     remote.connect(git2::Direction::Fetch)?;
    //     let rem_list = remote.list()?;
    //     for rem in rem_list {
    //         rem.name()
    //     }

    //     Ok(())
    // }
}
