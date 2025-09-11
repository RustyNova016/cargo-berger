use std::sync::LazyLock;
use std::sync::RwLock;

use crate::ColEyreVal;
use crate::models::config::WorkplaceConfig;
use crate::models::crate_data::CrateData;

pub static CLI_DATA: LazyLock<RwLock<CliData>> = LazyLock::new(|| RwLock::new(CliData::default()));

#[derive(Debug, Default)]
pub struct CliData {
    config_path: Option<String>,
}

impl CliData {
    pub fn set_config_path(&mut self, path: String) {
        self.config_path.replace(path);
    }

    pub fn get_crates_data(&self) -> ColEyreVal<Vec<CrateData>> {
        let conf = WorkplaceConfig::load(self.config_path.as_deref())?;

        let mut out = Vec::with_capacity(conf.crates.len());
        for crate_conf in conf.crates {
            out.push(CrateData::open_repo(crate_conf)?);
        }

        Ok(out)
    }
}
