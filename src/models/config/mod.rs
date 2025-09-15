pub mod rust_config;
use std::fs::File;
use std::io::Read as _;
use std::path::Path;

use color_eyre::eyre::Context as _;
use serde::Deserialize;
use serde::Serialize;

use crate::ColEyreVal;
use crate::models::config::repository_config::RepositoryConfig;

pub mod repository_config;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkplaceConfig {
    #[serde(
        alias = "crate",
        alias = "crates",
        alias = "repo",
        alias = "repos",
        alias = "repository",
        alias = "repositories"
    )]
    pub repositories: Vec<RepositoryConfig>,
}

impl WorkplaceConfig {
    pub fn load(path: &Path) -> ColEyreVal<Self> {
        let mut config = File::open(path)
            .context("Couldn't open the berger config file. Make sure it exists")?;
        let mut data = String::new();
        config
            .read_to_string(&mut data)
            .context("Couldn't read the autosort config file")?;
        toml::from_str(&data).context("Couldn't parse the berger config file")
    }
}
