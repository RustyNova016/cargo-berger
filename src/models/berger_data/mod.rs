use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use crate::ColEyreVal;
use crate::models::config::BergerConfig;
use crate::models::repository_data::RepositoryData;

/// RC Wrapper for [`BergerData`]
pub type BergerRc = Rc<BergerData>;

/// The compiled data comming from the berger config.
///
/// *You could say it's the **herd** of the app*
pub struct BergerData {
    /// Key value store of the Name + Repository data
    pub repo_data: HashMap<String, RepositoryData>,
}

impl BergerData {
    pub fn load(path: &Path) -> ColEyreVal<Self> {
        let conf = BergerConfig::load(path)?;
        Self::from_berger_config(conf)
    }

    /// Use the current folder as the only repo available. Used in case there's no berger file
    pub fn use_current() -> ColEyreVal<Self> {
        Self::from_berger_config(BergerConfig::use_current()?)
    }

    fn from_berger_config(conf: BergerConfig) -> Result<BergerData, color_eyre::eyre::Error> {
        let mut data = HashMap::new();

        for (name, crate_conf) in conf.repositories {
            data.insert(name.clone(), RepositoryData::open_repo(name, crate_conf)?);
        }

        Ok(Self { repo_data: data })
    }
}
