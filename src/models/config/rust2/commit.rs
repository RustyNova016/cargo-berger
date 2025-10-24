use serde::Deserialize;
use serde::Serialize;

use crate::utils::traits::merge_data::OverwriteMergeData;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RustCrateCommitConfig {
    #[serde(default)]
    pub fmt: bool,
    #[serde(default)]
    pub clippy: bool,
    #[serde(default)]
    pub clippy_hack: bool,
    #[serde(default)]
    pub sqlx: bool,
}

impl OverwriteMergeData for RustCrateCommitConfig {
    fn merge_data_mut(&mut self, other: Self) {
        self.fmt.merge_data_mut(other.fmt);
        self.clippy.merge_data_mut(other.clippy);
        self.clippy_hack.merge_data_mut(other.clippy_hack);
        self.sqlx.merge_data_mut(other.sqlx);
    }
}
