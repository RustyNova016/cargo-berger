use core::cell::OnceCell;
use std::collections::HashMap;
use std::env::current_dir;
use std::path::PathBuf;
use std::rc::Rc;

use crate::ColEyreVal;
use crate::models::berger_data::rust_workspace::RustWorkspace;
use crate::models::config::BergerConfig;
use crate::models::repository_data::RepositoryData;

pub mod rust_workspace;

/// RC Wrapper for [`BergerData`]
pub type BergerRc = Rc<BergerData>;

/// The compiled data comming from the berger config.
///
/// *You could say it's the **herd** of the app*
pub struct BergerData {
    /// Path to the root of the workspace;
    pub path: PathBuf,

    /// Key value store of the Name + Repository data
    pub repo_data: HashMap<String, RepositoryData>,

    rust_workspace: OnceCell<RustWorkspace>,
}

impl BergerData {
    pub fn load(path: PathBuf) -> ColEyreVal<Self> {
        let conf = BergerConfig::load(&path)?;

        let path = path
            .canonicalize()?
            .parent()
            .expect("Can't load a directory as a berger file")
            .to_path_buf();
        Self::from_berger_config(path, conf)
    }

    /// Use the current folder as the only repo available. Used in case there's no berger file
    pub fn use_current() -> ColEyreVal<Self> {
        Self::from_berger_config(current_dir()?, BergerConfig::use_current()?)
    }

    pub fn from_berger_config(
        path: PathBuf,
        conf: BergerConfig,
    ) -> Result<BergerData, color_eyre::eyre::Error> {
        let mut data = HashMap::new();

        for (name, crate_conf) in conf.repositories {
            data.insert(name.clone(), RepositoryData::open_repo(name, crate_conf)?);
        }

        Ok(Self {
            path,
            repo_data: data,
            rust_workspace: OnceCell::new(),
        })
    }

    pub fn get_rust_workspace(&self, create: bool) -> ColEyreVal<Option<&RustWorkspace>> {
        if !create {
            return Ok(self.rust_workspace.get());
        };

        //TODO: Rework this mess once rust gives us OnceCell::try_init()...

        if let Some(val) = self.rust_workspace.get() {
            return Ok(Some(val));
        }

        let rust = RustWorkspace::load_or_create(self.path.to_path_buf())?;
        Ok(Some(self.rust_workspace.get_or_init(|| rust)))
    }
}
