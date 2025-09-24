use std::path::PathBuf;
use std::rc::Rc;
use std::sync::LazyLock;
use std::sync::RwLock;

use crate::ColEyreVal;
use crate::models::berger_data::BergerData;
use crate::models::berger_data::BergerRc;
use crate::models::config::BergerConfig;

pub static CLI_DATA: LazyLock<RwLock<CliData>> = LazyLock::new(|| RwLock::new(CliData::default()));

#[derive(Debug)]
pub struct CliData {
    config_path: Option<String>,
    auto_init: bool,
}

impl CliData {
    pub fn set_config_path(&mut self, path: String) {
        self.config_path.replace(path);
    }

    pub fn set_auto_init(&mut self, val: bool) {
        self.auto_init = val;
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

    /// Load the configuration
    pub fn get_config(&self) -> ColEyreVal<Option<(PathBuf, BergerConfig)>> {
        match self.get_config_path() {
            Some(path) => {
                let path = path.canonicalize()?;
                let mut config = BergerConfig::load(&path)?;
                config.auto_init = config.auto_init && self.auto_init;

                Ok(Some((path, config)))
            }
            None => Ok(None),
        }
    }

    pub fn get_berger_data(&self) -> ColEyreVal<BergerRc> {
        let config = self.get_config()?;

        let data = match config {
            Some((path, config)) => {
                let workspace_root = path
                    .parent()
                    .expect("Couldn't find the parent folder of the config file");
                BergerData::from_berger_config(workspace_root.to_path_buf(), config)?
            }
            None => BergerData::use_current()?,
        };

        Ok(Rc::new(data))
    }
}

impl Default for CliData {
    fn default() -> Self {
        Self {
            auto_init: true,
            config_path: Default::default(),
        }
    }
}
