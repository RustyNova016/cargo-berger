use serde::Deserialize;
use serde::Serialize;

use crate::models::config::ci::rust::RustCIConfig;

pub mod rust;
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CIConfig {
    #[serde(default = "default_full_commit")]
    pub full_commit: bool,

    #[serde(default)]
    pub rust_ci: RustCIConfig,
}

fn default_full_commit() -> bool {
    true
}

impl Default for CIConfig {
    fn default() -> Self {
        Self {
            full_commit: default_full_commit(),
            rust_ci: Default::default(),
        }
    }
}
