use std::fs::File;
use std::io;
use std::io::Read as _;
use std::path::Path;

use snafu::OptionExt;
use snafu::ResultExt;

use crate::models::config::BergerConfig;
use crate::models::config::repository_config::RepositoryConfig;
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
        for (name, repo) in self.repositories.iter_mut() {
            if !repo.inherit {
                continue;
            }

            match repo.berger_file_path() {
                Some(inherited_config_path) => {
                    Self::load_explicit_config(&self.from_path, repo, name, &inherited_config_path)?
                }
                None => Self::load_implicit_config(&self.from_path, repo, name)?,
            }
        }

        Ok(())
    }

    /// Load an explicit config
    fn load_explicit_config(
        config_path: &Path,
        current_master_repo: &mut RepositoryConfig,
        name: &str,
        inherited_config_path: &Path,
    ) -> Result<(), BergerConfigLoadInheritedError> {
        // The config contains a relative path to the inherited file. Let's join the dir of self's, and that new path
        let inherited_config_path = config_path.parent().unwrap().join(inherited_config_path);

        let inherited_config = match Self::load(&inherited_config_path) {
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

        Self::merge_repo_config(name, current_master_repo, &inherited_config).context(
            MergeRepositoryDataSnafu {
                path: inherited_config_path.display().to_string(),
            },
        )?;

        Ok(())
    }

    /// Load an implicit config
    fn load_implicit_config(
        config_path: &Path,
        current_master_repo: &mut RepositoryConfig,
        name: &str,
    ) -> Result<(), BergerConfigLoadInheritedError> {
        // No explicite path has been given. Let's join the dir of self's, and the repo's berger file
        let self_dir = config_path.parent().unwrap();

        let inherited_config_path = self_dir
            .join(current_master_repo.path())
            .join("berger.toml");

        if !inherited_config_path.exists() {
            return Ok(());
        }

        // Check if the inherited file is ourself
        if config_path.canonicalize().unwrap() == inherited_config_path.canonicalize().unwrap() {
            return Ok(());
        }

        println!("self: {}", config_path.canonicalize().unwrap().display());
        println!(
            "inherited_config_path: {}",
            inherited_config_path.canonicalize().unwrap().display()
        );

        let Some(inherited_config) = Self::load_if_exists(&inherited_config_path)
            .map_err(Box::new)
            .context(ConfigLoadSnafu {
                path: inherited_config_path.display().to_string(),
            })?
        else {
            return Ok(());
        };

        Self::merge_repo_config(name, current_master_repo, &inherited_config).context(
            MergeRepositoryDataSnafu {
                path: inherited_config_path.display().to_string(),
            },
        )?;

        Ok(())
    }

    pub fn merge_repo_config(
        name: &str,
        current_master_repo: &mut RepositoryConfig,
        inherited_config: &BergerConfig,
    ) -> Result<(), MissingRepoInInheritedError> {
        let inherited_repo_config =
            inherited_config
                .repositories
                .get(name)
                .context(MissingRepoInInheritedSnafu {
                    name: name.to_string(),
                })?;

        let new_repo_config = inherited_repo_config
            .to_owned()
            .merge_data(current_master_repo.to_owned());
        *current_master_repo = new_repo_config;

        Ok(())
    }
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
