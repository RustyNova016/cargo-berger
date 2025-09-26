use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RustCIConfig {
    #[serde(default = "default_fmt")]
    pub fmt: bool,

    #[serde(default = "default_clippy")]
    pub clippy: bool,
}

fn default_fmt() -> bool {
    true
}

fn default_clippy() -> bool {
    false
}

impl Default for RustCIConfig {
    fn default() -> Self {
        Self {
            fmt: default_fmt(),
            clippy: default_clippy(),
        }
    }
}
