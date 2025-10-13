use std::collections::HashMap;
use std::path::PathBuf;

use color_eyre::eyre::Ok;
use serde::Deserialize;
use serde::Serialize;

use crate::ColEyreVal;
use crate::models::config::repository_config::RepositoryConfig;

pub mod load;
pub mod release;
pub mod repository_config;
pub mod rust;

/// Configuration root
#[derive(Debug, Serialize, Deserialize)]
pub struct BergerConfig {
    #[serde(
        alias = "crate",
        alias = "crates",
        alias = "repo",
        alias = "repos",
        alias = "repository",
        alias = "repositories"
    )]
    pub repositories: HashMap<String, RepositoryConfig>,

    #[serde(default = "default_auto_init")]
    pub auto_init: bool,

    #[serde(skip_deserializing)]
    pub from_path: PathBuf,
}

impl BergerConfig {
    /// Use the current folder as the only repo available. Used in case there's no berger file
    pub fn use_current() -> ColEyreVal<Self> {
        let repo_conf = RepositoryConfig::new("./".to_string());

        let mut repos = HashMap::new();
        repos.insert("current".to_string(), repo_conf);

        Ok(BergerConfig {
            repositories: repos,
            auto_init: false,
            from_path: PathBuf::from("./"),
        })
    }
}

fn default_auto_init() -> bool {
    true
}
