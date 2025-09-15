use serde::Deserialize;
use serde::Serialize;

use crate::models::config::rust_config::RustConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryConfig {
    pub name: String,
    pub path: String,

    pub remote_url: Option<String>,

    // Git settings
    #[serde(alias = "remote", default = "default_remote")]
    pub default_remote: String,
    #[serde(default = "default_branch")]
    pub default_branch: String,

    // Languages
    pub rust: Option<RustConfig>,
}

impl RepositoryConfig {
    pub fn new(name: String, path: String) -> Self {
        Self {
            name,
            path,
            remote_url: None,
            default_branch: default_branch(),
            default_remote: default_remote(),
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
