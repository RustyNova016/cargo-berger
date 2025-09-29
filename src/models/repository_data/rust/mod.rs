use std::path::Path;

use crate::ColEyre;
use crate::ColEyreVal;
use crate::models::config::rust::RustConfig;
use crate::models::tool_bindings::cargo::Cargo;
use crate::models::tool_bindings::cargo::cargo_file::CargoFile;

/// Handle all the rust configuration and action for the repo
pub struct RustData {
    #[expect(dead_code)]
    cargo_file: CargoFile,
    rust_conf: RustConfig,

    cargo: Cargo,
}

impl RustData {
    pub fn load(directory: &Path, rust_conf: RustConfig) -> ColEyreVal<Self> {
        Ok(Self {
            cargo_file: CargoFile::load(directory.join("Cargo.toml"))?,
            rust_conf,
            cargo: Cargo::new(directory.to_path_buf()),
        })
    }

    pub fn pre_full_commit(&self) -> ColEyre {
        if self.rust_conf.fmt {
            println!("\n === Running Formater ===\n");
            self.cargo.fmt()?;
        }

        if self.rust_conf.sqlx {
            println!("\n === Running sqlx prepare ===\n");
            self.cargo.sqlx_prepare()?;
        }

        if self.rust_conf.clippy_hack {
            println!("\n === Running Clippy Hack ===\n");
            self.cargo.clippy_hack()?;
        }

        Ok(())
    }

    pub fn run_ci(&self) -> ColEyre {
        if self.rust_conf.ci.fmt {
            println!("\n === Running formater check ===\n");
            self.cargo.fmt_check()?;
        }

        if self.rust_conf.ci.doc {
            println!("\n === Running documentation check ===\n");
            self.cargo.fmt_check()?;
        }

        if self.rust_conf.ci.clippy {
            println!("\n === Running clippy check ===\n");
            self.cargo.clippy()?;
        }

        if self.rust_conf.ci.test {
            println!("\n === Running tests ===\n");
            if self.rust_conf.ci.nextest {
                self.cargo.nextest_run()?;
            } else {
                self.cargo.test()?;
            }
        }

        if self.rust_conf.ci.semver {
            println!("\n === Running semver check ===\n");
            self.cargo.cargo_semver()?;
        }

        if self.rust_conf.ci.machete {
            println!("\n === Running machete check ===\n");
            self.cargo.cargo_machete()?;
        }

        if self.rust_conf.ci.min_versions {
            println!("\n === Running minimum version check ===\n");
            self.cargo.cargo_min_version()?;
        }

        if self.rust_conf.ci.msrv {
            println!("\n === Running msrv check ===\n");
            let res = self.cargo.cargo_msrv_verify();

            if res.is_err() && self.rust_conf.ci.msrv_find {
                println!("\n === Finding new msrv ===\n");
                self.cargo.cargo_msrv_find()?;
                return res;
            }
        }

        Ok(())
    }
}
