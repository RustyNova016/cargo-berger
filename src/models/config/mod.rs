use std::fs::File;
use std::io::Read as _;
use std::path::Path;

use color_eyre::eyre::Context as _;
use serde::Deserialize;
use serde::Serialize;

use crate::ColEyreVal;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkplaceConfig {
    pub crates: Vec<CrateConfig>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CrateConfig {
    pub name: String,
    pub path: String,
    pub default_branch: String,

    #[serde(default)]
    pub sqlx: bool,
}
