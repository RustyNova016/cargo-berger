use std::collections::HashMap;
use std::path::PathBuf;

use color_eyre::eyre::Ok;
use serde::Deserialize;
use serde::Serialize;

use crate::ColEyreVal;
use crate::models::config::repository_config::RepositoryConfig;
use crate::models::config::rust2::RustCrateConfig;
use crate::utils::traits::merge_data::OverwriteMergeData;

pub mod load;
pub mod release;
pub mod repository_config;
pub mod rust;
pub mod rust2;

/// Configuration root
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BergerConfig {
    #[serde(default = "default_auto_init")]
    pub auto_init: bool,

    #[serde(
        alias = "repo",
        alias = "repos",
        alias = "repository",
        alias = "repositories",
        default
    )]
    pub repositories: HashMap<String, RepositoryConfig>,

    #[serde(alias = "crate", alias = "crates", default)]
    pub crates: HashMap<String, RustCrateConfig>,

    // === inner data ===
    /// Where this config as been loaded
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
            crates: Default::default(),
        })
    }
}

impl OverwriteMergeData for BergerConfig {
    fn merge_data_mut(&mut self, other: Self)
    where
        Self: Sized,
    {
        self.auto_init.merge_data_mut(other.auto_init);
        self.from_path.merge_data_mut(other.from_path);
        self.repositories.merge_data_mut(other.repositories);
        self.crates.merge_data_mut(other.crates);
    }
}

fn default_auto_init() -> bool {
    true
}
