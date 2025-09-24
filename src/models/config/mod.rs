pub mod rust_config;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read as _;
use std::path::Path;

use color_eyre::eyre::Context as _;
use serde::Deserialize;
use serde::Serialize;

use crate::ColEyreVal;
use crate::models::config::repository_config::RepositoryConfig;

pub mod repository_config;

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
}

impl BergerConfig {
    pub fn load(path: &Path) -> ColEyreVal<Self> {
        let mut config = File::open(path)
            .context("Couldn't open the berger config file. Make sure it exists")?;
        let mut data = String::new();
        config
            .read_to_string(&mut data)
            .context("Couldn't read the autosort config file")?;
        toml::from_str(&data).context("Couldn't parse the berger config file")
    }

    /// Use the current folder as the only repo available. Used in case there's no berger file
    pub fn use_current() -> ColEyreVal<Self> {
        let repo_conf = RepositoryConfig::new("./".to_string());

        let mut repos = HashMap::new();
        repos.insert("current".to_string(), repo_conf);

        Ok(BergerConfig {
            repositories: repos,
            auto_init: false,
        })
    }
}

fn default_auto_init() -> bool {
    true
}
