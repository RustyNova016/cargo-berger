pub mod release;

use serde::Deserialize;
use serde::Serialize;

use crate::models::config::repository::release::ReleaseConfig;
use crate::models::config::rust::RustConfig;

/// Configuration for a repository in the herd
#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryConfig {
    pub path: String,

    pub remote_url: Option<String>,

    // Git settings
    #[serde(alias = "remote", default = "default_remote")]
    pub default_remote: String,
    #[serde(default = "default_branch")]
    pub default_branch: String,

    /// Release config
    #[serde(default)]
    pub release: ReleaseConfig,

    // Languages
    pub rust: Option<RustConfig>,
}

impl RepositoryConfig {
    pub fn new(path: String) -> Self {
        Self {
            path,
            remote_url: None,
            default_branch: default_branch(),
            default_remote: default_remote(),
            release: Default::default(),
            rust: None,
        }
    }
}

fn default_remote() -> String {
    "origin".to_string()
}

fn default_branch() -> String {
    "master".to_string()
}
