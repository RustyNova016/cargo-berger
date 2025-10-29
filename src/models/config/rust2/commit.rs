use serde::Deserialize;
use serde::Serialize;

use crate::utils::traits::merge_data::OverwriteMergeData;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RustCrateCommitConfig {
    fmt: Option<bool>,
    clippy: Option<bool>,
    clippy_hack: Option<bool>,
    sqlx: Option<bool>,
}

impl RustCrateCommitConfig {
    pub fn fmt(&self) -> bool {
        self.fmt.unwrap_or(false)
    }

    pub fn clippy(&self) -> bool {
        self.clippy.unwrap_or(false)
    }

    pub fn clippy_hack(&self) -> bool {
        self.clippy_hack.unwrap_or(false)
    }

    pub fn sqlx(&self) -> bool {
        self.sqlx.unwrap_or(false)
    }
}

impl OverwriteMergeData for RustCrateCommitConfig {
    fn merge_data_mut(&mut self, other: Self) {
        self.fmt.merge_data_mut(other.fmt);
        self.clippy.merge_data_mut(other.clippy);
        self.clippy_hack.merge_data_mut(other.clippy_hack);
        self.sqlx.merge_data_mut(other.sqlx);
    }
}
