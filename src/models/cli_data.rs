use std::path::PathBuf;
use std::rc::Rc;
use std::sync::LazyLock;
use std::sync::RwLock;

use crate::ColEyreVal;
use crate::models::berger_data::BergerData;
use crate::models::berger_data::BergerRc;

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

    pub fn get_berger_data(&self) -> ColEyreVal<BergerRc> {
        let config_path = self.get_config_path();

        let data = match config_path {
            Some(path) => BergerData::load(path)?,
            None => BergerData::use_current()?,
        };

        Ok(Rc::new(data))
    }
}
