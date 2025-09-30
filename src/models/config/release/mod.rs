use serde::Deserialize;
use serde::Serialize;

/// Configuration for a repository in the herd
#[derive(Debug, Serialize, Deserialize)]
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

fn default_git_cliff() -> bool {
    false
}
