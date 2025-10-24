use serde::Deserialize;
use serde::Serialize;

use crate::utils::traits::merge_data::OverwriteMergeData;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RustCrateCIConfig {
    #[serde(default = "default_fmt")]
    pub fmt: bool,

    #[serde(default = "default_doc")]
    pub doc: bool,

    #[serde(default = "default_clippy")]
    pub clippy: bool,

    #[serde(default = "default_test")]
    pub test: bool,

    #[serde(default = "default_nextest")]
    pub nextest: bool,

    #[serde(default = "default_msrv")]
    pub msrv: bool,

    #[serde(default = "default_msrv_find")]
    pub msrv_find: bool,

    #[serde(default = "default_machete")]
    pub machete: bool,

    #[serde(default = "default_semver")]
    pub semver: bool,

    #[serde(default = "default_min_versions")]
    pub min_versions: bool,
}

fn default_fmt() -> bool {
    true
}

fn default_doc() -> bool {
    true
}

fn default_clippy() -> bool {
    true
}

fn default_test() -> bool {
    true
}

fn default_nextest() -> bool {
    false
}

fn default_msrv() -> bool {
    false
}

fn default_msrv_find() -> bool {
    true
}

fn default_machete() -> bool {
    false
}

fn default_semver() -> bool {
    false
}

fn default_min_versions() -> bool {
    false
}

impl Default for RustCrateCIConfig {
    fn default() -> Self {
        Self {
            fmt: default_fmt(),
            doc: default_doc(),
            clippy: default_clippy(),
            test: default_test(),
            nextest: default_nextest(),
            msrv: default_msrv(),
            msrv_find: default_msrv_find(),
            machete: default_msrv_find(),
            semver: default_semver(),
            min_versions: default_min_versions(),
        }
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
