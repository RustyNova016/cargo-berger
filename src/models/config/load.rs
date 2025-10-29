use std::fs::File;
use std::io;
use std::io::Read as _;
use std::path::Path;
use std::path::PathBuf;

use itertools::Itertools;
use snafu::ResultExt;

use crate::models::config::BergerConfig;
use crate::utils::traits::merge_data::OverwriteMergeData as _;

impl BergerConfig {
    pub fn load_if_exists(path: &Path) -> Result<Option<Self>, BergerConfigLoadError> {
        match Self::load(path) {
            Ok(val) => Ok(Some(val)),
            Err(BergerConfigLoadError::FileNotFound { path: _ }) => Ok(None),
            Err(err) => Err(err),
        }
    }

    pub fn load(config_path: &Path) -> Result<Self, BergerConfigLoadError> {
        if !config_path.exists() {
            return FileNotFoundSnafu {
                path: config_path.display().to_string(),
            }
            .fail();
        }

        let mut config = File::open(config_path).context(IoSnafu)?;
        let mut data = String::new();

        config.read_to_string(&mut data).context(IoSnafu)?;

        let mut data: Self = toml::from_str(&data).context(ParsingSnafu {
            path: config_path.display().to_string(),
        })?;

        data.from_path = config_path.to_path_buf();

        data.load_inherited().context(InheritedLoadSnafu {
            path: config_path.display().to_string(),
        })?;

        Ok(data)
    }

    /// Load all the inherited repos
    pub fn load_inherited(&mut self) -> Result<(), BergerConfigLoadInheritedError> {
        let paths = self
            .get_inherited_repository_paths()
            .into_iter()
            .chain(self.get_inherited_crates_paths())
            .unique()
            .collect_vec();

        self.load_inherited_configs_path(paths)
    }

    /// Get the paths for the inherited repositories
    pub(self) fn get_inherited_repository_paths(&self) -> Vec<ConfigPathType> {
        self.repositories
            .values()
            .filter(|repo| repo.inherit)
            .filter_map(|repo| match repo.berger_file_path() {
                Some(inherited_config_path) => Some(ConfigPathType::Explicit(
                    self.from_path.parent().unwrap().join(inherited_config_path),
                )),
                None => Self::get_implicit_config_path(&self.from_path, &repo.path())
                    .map(ConfigPathType::Implicit),
            })
            .collect_vec()
    }

    /// Get the paths for the inherited rust crates
    pub(self) fn get_inherited_crates_paths(&self) -> Vec<ConfigPathType> {
        self.crates
            .values()
            .filter(|crat| crat.inherit())
            .filter_map(|crat| match crat.berger_file_path() {
                Some(inherited_config_path) => Some(ConfigPathType::Explicit(
                    self.from_path.parent().unwrap().join(inherited_config_path),
                )),
                None => Self::get_implicit_config_path(&self.from_path, &crat.path())
                    .map(ConfigPathType::Implicit),
            })
            .collect_vec()
    }

    pub(self) fn load_inherited_configs_path(
        &mut self,
        configs: Vec<ConfigPathType>,
    ) -> Result<(), BergerConfigLoadInheritedError> {
        for path in configs {
            match path {
                ConfigPathType::Explicit(path) => self.load_explicit_config(&path)?,
                ConfigPathType::Implicit(path) => self.load_implicit_config(&path)?,
            }
        }

        Ok(())
    }

    /// Load an explicitly linked config
    fn load_explicit_config(
        &mut self,
        inherited_config_path: &Path,
    ) -> Result<(), BergerConfigLoadInheritedError> {
        let inherited_config = match Self::load(inherited_config_path) {
            Ok(val) => val,
            Err(BergerConfigLoadError::FileNotFound { path: _ }) => {
                return MissingExplicitInheritSnafu {
                    target_config: inherited_config_path.display().to_string(),
                }
                .fail();
            }
            Err(err) => {
                return Err(Box::new(err)).context(ConfigLoadSnafu {
                    path: inherited_config_path.display().to_string(),
                });
            }
        };

        Self::merge_repo_config(self, &inherited_config).context(MergeRepositoryDataSnafu {
            path: inherited_config_path.display().to_string(),
        })?;

        Ok(())
    }

    /// Generate an implicite config path for a repo.
    ///
    /// The parent config path represent the path of the config we are looking for implicits, and repo_path is the path of the current repo looked at
    pub fn get_implicit_config_path(
        parent_config_path: &Path,
        repo_path: &Path,
    ) -> Option<PathBuf> {
        let self_dir = parent_config_path.parent().unwrap();

        let inherited_config_path = self_dir.join(repo_path).join("berger.toml");

        if !inherited_config_path.exists() {
            return None;
        }

        // Check if the inherited file is ourself
        if parent_config_path.canonicalize().unwrap()
            == inherited_config_path.canonicalize().unwrap()
        {
            return None;
        }

        Some(inherited_config_path)
    }

    /// Load an implicit config
    fn load_implicit_config(
        &mut self,
        implicit_config_path: &Path,
    ) -> Result<(), BergerConfigLoadInheritedError> {
        let Some(inherited_config) = Self::load_if_exists(implicit_config_path)
            .map_err(Box::new)
            .context(ConfigLoadSnafu {
                path: implicit_config_path.display().to_string(),
            })?
        else {
            return Ok(());
        };

        Self::merge_repo_config(self, &inherited_config).context(MergeRepositoryDataSnafu {
            path: implicit_config_path.display().to_string(),
        })?;

        Ok(())
    }

    /// Merge two berger files together, with the parent one overwriting the inherited
    pub fn merge_repo_config(
        parent_berger_conf: &mut BergerConfig,
        inherited_config: &BergerConfig,
    ) -> Result<(), MissingRepoInInheritedError> {
        let new_repo_config = inherited_config
            .to_owned()
            .merge_data(parent_berger_conf.to_owned());

        *parent_berger_conf = new_repo_config;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ConfigPathType {
    Explicit(PathBuf),
    Implicit(PathBuf),
}

#[derive(Debug, snafu::Snafu)]
pub enum BergerConfigLoadError {
    #[snafu(display("Tried to load berger config at `{path}` but the file doesn't exists"))]
    FileNotFound { path: String },

    #[snafu(display("IO Error while getting the configuration"))]
    IoError { source: io::Error },

    #[snafu(display("Couldn't parse the berger file at `{path}`"))]
    ParsingError {
        path: String,
        source: toml::de::Error,
    },

    #[snafu(display("When loading inherited configs of `{path}`"))]
    InheritedLoadError {
        path: String,
        source: BergerConfigLoadInheritedError,
    },
}

#[derive(Debug, snafu::Snafu)]
pub enum BergerConfigLoadInheritedError {
    #[snafu(display(
        "Found an explicitly inherited config path, but path `{target_config}` doesn't exists"
    ))]
    MissingExplicitInheritError { target_config: String },

    #[snafu(display("{source} in config `{path}`"))]
    MergeRepositoryData {
        source: MissingRepoInInheritedError,
        path: String,
    },

    #[snafu(display("Couldn't load inherited configuration at `{path}`"))]
    ConfigLoadError {
        path: String,
        source: Box<BergerConfigLoadError>,
    },
}

#[derive(Debug, snafu::Snafu)]
#[snafu(display("Missing repository `{name}`"))]
pub struct MissingRepoInInheritedError {
    name: String,
}
