use serde::Deserialize;
use serde::Serialize;

use crate::utils::traits::merge_data::OverwriteMergeData;

/// Configuration for a repository in the herd
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReleaseConfig {
    #[serde(default = "default_git_cliff")]
    pub git_cliff: bool,
}

impl Default for ReleaseConfig {
    fn default() -> Self {
        Self {
            git_cliff: default_git_cliff(),
        }
    }
}

impl OverwriteMergeData for ReleaseConfig {
    fn merge_data_mut(&mut self, other: Self) {
        self.git_cliff.merge_data_mut(other.git_cliff);
    }
}

fn default_git_cliff() -> bool {
    false
}
