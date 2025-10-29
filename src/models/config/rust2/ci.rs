use serde::Deserialize;
use serde::Serialize;

use crate::utils::traits::merge_data::OverwriteMergeData;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RustCrateCIConfig {
    fmt: Option<bool>,
    doc: Option<bool>,
    clippy: Option<bool>,
    test: Option<bool>,
    nextest: Option<bool>,
    msrv: Option<bool>,
    msrv_find: Option<bool>,
    machete: Option<bool>,
    semver: Option<bool>,
    min_versions: Option<bool>,
}

impl RustCrateCIConfig {
    pub fn fmt(&self) -> bool {
        self.fmt.unwrap_or(true)
    }
    pub fn doc(&self) -> bool {
        self.doc.unwrap_or(true)
    }
    pub fn clippy(&self) -> bool {
        self.clippy.unwrap_or(true)
    }
    pub fn test(&self) -> bool {
        self.test.unwrap_or(true)
    }
    pub fn nextest(&self) -> bool {
        self.nextest.unwrap_or(false)
    }
    pub fn msrv(&self) -> bool {
        self.msrv.unwrap_or(false)
    }
    pub fn msrv_find(&self) -> bool {
        self.msrv_find.unwrap_or(true)
    }
    pub fn machete(&self) -> bool {
        self.machete.unwrap_or(false)
    }
    pub fn semver(&self) -> bool {
        self.semver.unwrap_or(false)
    }
    pub fn min_versions(&self) -> bool {
        self.min_versions.unwrap_or(false)
    }
}

impl OverwriteMergeData for RustCrateCIConfig {
    fn merge_data_mut(&mut self, other: Self) {
        self.fmt.merge_data_mut(other.fmt);
        self.doc.merge_data_mut(other.doc);
        self.clippy.merge_data_mut(other.clippy);
        self.test.merge_data_mut(other.test);
        self.nextest.merge_data_mut(other.nextest);
        self.msrv.merge_data_mut(other.msrv);
        self.msrv_find.merge_data_mut(other.msrv_find);
        self.machete.merge_data_mut(other.msrv_find);
        self.semver.merge_data_mut(other.msrv_find);
        self.min_versions.merge_data_mut(other.msrv_find);
    }
}
