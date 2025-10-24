use core::cell::OnceCell;
use std::collections::HashMap;
use std::env::current_dir;
use std::path::PathBuf;
use std::process::exit;
use std::rc::Rc;

use crate::ColEyreVal;
use crate::errorln;
use crate::models::berger_data::rust_workspace::RustWorkspace;
use crate::models::config::BergerConfig;
use crate::models::repository_data::RepositoryData;

pub mod git;
pub mod rust_workspace;

/// RC Wrapper for [`BergerData`]
pub type BergerRc = Rc<BergerData>;

/// The compiled data comming from the berger config.
///
/// *You could say it's the **herd** of the app*
pub struct BergerData {
    /// Path to the root of the workspace;
    pub workspace_root: PathBuf,

    /// Key value store of the Name + Repository data
    pub repo_data: HashMap<String, RepositoryData>,

    rust_workspace: OnceCell<RustWorkspace>,
}

impl BergerData {
    /// Use the current folder as the only repo available. Used in case there's no berger file
    pub fn use_current() -> ColEyreVal<Self> {
        Self::from_berger_config(current_dir()?, BergerConfig::use_current()?)
    }

    pub fn from_berger_config(
        berger_root: PathBuf,
        conf: BergerConfig,
    ) -> Result<BergerData, color_eyre::eyre::Error> {
        let mut data = HashMap::new();

        for (name, crate_conf) in conf.repositories {
            let rust_conf = conf.crates.get(&name).cloned();

            let repo = if conf.auto_init {
                RepositoryData::initialize_repo(name.clone(), crate_conf, rust_conf, &berger_root)?
            } else {
                RepositoryData::load(name.clone(), crate_conf, rust_conf, &berger_root)?
            };

            data.insert(name.clone(), repo);
        }

        Ok(Self {
            workspace_root: berger_root,
            repo_data: data,
            rust_workspace: OnceCell::new(),
        })
    }

    pub fn get_rust_workspace(&self, create: bool) -> ColEyreVal<Option<&RustWorkspace>> {
        //TODO: Rework this mess once rust gives us OnceCell::try_init()...

        if let Some(val) = self.rust_workspace.get() {
            return Ok(Some(val));
        }

        let rust = match RustWorkspace::load(self.workspace_root.to_path_buf(), create)? {
            Some(wp) => wp,
            None => return Ok(None),
        };

        Ok(Some(self.rust_workspace.get_or_init(|| rust)))
    }

    pub fn get_repository_or_exit(&self, repo: &str) -> &RepositoryData {
        match self.repo_data.get(repo) {
            Some(val) => val,
            None => {
                errorln!("Couldn't find repository `{repo}` in the workspace");
                exit(1);
            }
        }
    }
}
