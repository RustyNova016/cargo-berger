use std::path::PathBuf;
use std::sync::LazyLock;
use std::sync::RwLock;

use crate::ColEyreVal;
use crate::models::config::WorkplaceConfig;
use crate::models::config::repository_config::RepositoryConfig;
use crate::models::repository_data::RepositoryData;

pub static CLI_DATA: LazyLock<RwLock<CliData>> = LazyLock::new(|| RwLock::new(CliData::default()));

#[derive(Debug, Default)]
pub struct CliData {
    config_path: Option<String>,
}

impl CliData {
    pub fn set_config_path(&mut self, path: String) {
        self.config_path.replace(path);
    }

    /// Get the config file's path, returned the provided, or the one in the current directory, or the one in the parent directory
    pub fn get_config_path(&self) -> Option<PathBuf> {
        if let Some(path) = &self.config_path {
            return Some(PathBuf::from(path));
        }

        let path = PathBuf::from("./berger.toml");
        if path.exists() {
            return Some(path);
        }

        let path = PathBuf::from("../berger.toml");
        if path.exists() {
            return Some(path);
        }

        None
    }

    pub fn get_crates_data(&self) -> ColEyreVal<Vec<RepositoryData>> {
        let config_path = self.get_config_path();

        let conf = match config_path {
            Some(path) => WorkplaceConfig::load(&path)?,
            None => {
                let repo_conf =
                    RepositoryConfig::new("Current Folder".to_string(), "./".to_string());

                WorkplaceConfig {
                    repositories: vec![repo_conf],
                }
            }
        };

        let mut out = Vec::with_capacity(conf.repositories.len());
        for crate_conf in conf.repositories {
            out.push(RepositoryData::open_repo(crate_conf)?);
        }

        Ok(out)
    }
}
