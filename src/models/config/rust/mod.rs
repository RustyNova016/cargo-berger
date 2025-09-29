use serde::Deserialize;
use serde::Serialize;

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
}
