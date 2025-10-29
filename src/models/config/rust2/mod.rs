use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::models::config::rust::ci::RustCIConfig;
use crate::models::config::rust2::commit::RustCrateCommitConfig;
use crate::utils::traits::merge_data::OverwriteMergeData;

pub mod ci;
pub mod commit;

/// Configuration for a rust crate in the herd
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RustCrateConfig {
    pub path: String,

    pub inherit: Option<bool>,
    berger_file_path: Option<String>,

    #[serde(default)]
    pub commit: RustCrateCommitConfig,

    #[serde(default)]
    pub ci: RustCIConfig,
}

impl RustCrateConfig {
    pub fn path(&self) -> PathBuf {
        PathBuf::from(self.path.clone())
    }

    pub fn berger_file_path(&self) -> Option<PathBuf> {
        self.berger_file_path.as_ref().map(PathBuf::from)
    }

    pub fn inherit(&self) -> bool {
        self.inherit.unwrap_or(true)
    }
}

impl OverwriteMergeData for RustCrateConfig {
    fn merge_data_mut(&mut self, other: Self) {
        self.path.merge_data_mut(other.path);
        self.commit.merge_data_mut(other.commit);
        self.ci.merge_data_mut(other.ci);
    }
}
