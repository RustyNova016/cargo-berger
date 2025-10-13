use serde::Deserialize;
use serde::Serialize;

use crate::models::config::rust::ci::RustCIConfig;
use crate::utils::traits::merge_data::OverwriteMergeData;

pub mod ci;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RustConfig {
    #[serde(default)]
    pub fmt: bool,
    #[serde(default)]
    pub clippy: bool,
    #[serde(default)]
    pub clippy_hack: bool,
    #[serde(default)]
    pub sqlx: bool,

    /// What crates to patch for this dependancy
    #[serde(default)]
    pub require_patch: Vec<String>,

    #[serde(default)]
    pub ci: RustCIConfig,
}

impl OverwriteMergeData for RustConfig {
    fn merge_data_mut(&mut self, other: Self) {
        self.fmt.merge_data_mut(other.fmt);
        self.clippy.merge_data_mut(other.clippy);
        self.clippy_hack.merge_data_mut(other.clippy_hack);
        self.sqlx.merge_data_mut(other.sqlx);

        if !other.require_patch.is_empty() {
            self.require_patch = other.require_patch
        }
        self.ci.merge_data_mut(other.ci);
    }
}
