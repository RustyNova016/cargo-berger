use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RustCIConfig {
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

impl Default for RustCIConfig {
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
