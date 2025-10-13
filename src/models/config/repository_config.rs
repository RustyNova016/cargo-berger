use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::models::config::release::ReleaseConfig;
use crate::models::config::rust::RustConfig;
use crate::utils::traits::merge_data::OverwriteMergeData;

/// Configuration for a repository in the herd
#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryConfig {
    pub path: String,

    // === Berger files settings ===
    #[serde(default = "default_inherit")]
    pub inherit: bool,
    berger_file_path: Option<String>,

    // Git settings
    pub remote_url: Option<String>,
    #[serde(alias = "remote", default = "default_remote")]
    pub default_remote: String,
    #[serde(default = "default_branch")]
    pub default_branch: String,

    // Release settings
    pub release: Option<ReleaseConfig>,

    // Languages
    pub rust: Option<RustConfig>,
}

impl RepositoryConfig {
    pub fn new(path: String) -> Self {
        Self {
            path,
            berger_file_path: None,
            inherit: true,
            remote_url: None,
            default_branch: default_branch(),
            default_remote: default_remote(),
            release: Default::default(),
            rust: None,
        }
    }

    pub fn path(&self) -> PathBuf {
        PathBuf::from(self.path.clone())
    }

    pub fn full_path(&self, wp_root: &Path) -> PathBuf {
        wp_root.join(&self.path)
    }

    pub fn berger_file_path(&self) -> PathBuf {
        match &self.berger_file_path {
            Some(path) => PathBuf::from(path),
            None => self.path(),
        }
    }
}

impl OverwriteMergeData for RepositoryConfig {
    fn merge_data_mut(&mut self, other: Self) {
        self.remote_url.merge_data_mut(other.remote_url);
        self.default_remote.merge_data_mut(other.default_remote);
        self.default_branch.merge_data_mut(other.default_branch);
        self.release.merge_data_mut(other.release);
        self.rust.merge_data_mut(other.rust);
    }
}

fn default_inherit() -> bool {
    true
}

fn default_remote() -> String {
    "origin".to_string()
}

fn default_branch() -> String {
    "master".to_string()
}
