use crate::ColEyre;
use crate::ColEyreVal;
use crate::models::repository_data::RepositoryData;
use crate::models::tool_bindings::cargo::Cargo;

impl RepositoryData {
    pub fn get_cargo_runner(&self) -> ColEyreVal<Cargo> {
        Ok(Cargo::new(self.get_directory()?))
    }

    pub fn rust_precommit_checks(&self) -> ColEyre {
        let Some(rust_conf) = &self.conf.rust else {
            return Ok(());
        };
        let cargo = self.get_cargo_runner()?;

        if rust_conf.fmt {
            println!("\n === Running Formater ===\n");
            cargo.fmt()?;
        }

        if rust_conf.sqlx {
            println!("\n === Running sqlx prepare ===\n");
            cargo.sqlx_prepare()?;
        }

        if rust_conf.clippy_hack {
            println!("\n === Running Clippy Hack ===\n");
            cargo.clippy_hack()?;
        }

        Ok(())
    }
}
