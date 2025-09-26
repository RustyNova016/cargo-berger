use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RustCIConfig {
    #[serde(default = "default_fmt")]
    pub fmt: bool,

    #[serde(default = "default_clippy")]
    pub clippy: bool,

    #[serde(default = "default_msrv")]
    pub msrv: bool,

    #[serde(default = "default_msrv_find")]
    pub msrv_find: bool,
}

fn default_fmt() -> bool {
    true
}

fn default_clippy() -> bool {
    false
}

fn default_msrv() -> bool {
    false
}

fn default_msrv_find() -> bool {
    true
}

impl Default for RustCIConfig {
    fn default() -> Self {
        Self {
            fmt: default_fmt(),
            clippy: default_clippy(),
            msrv: default_msrv(),
            msrv_find: default_msrv_find(),
        }
    }
}
