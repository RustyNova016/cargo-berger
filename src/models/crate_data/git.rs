use crate::ColEyre;
use crate::models::crate_data::CrateData;

impl CrateData {
    pub fn make_tmp_save_commit(&self, message: Option<&str>) -> ColEyre {
        if !self.repository.is_dirty()? {
            println!("[ Crate `{}` is clean. Skipping commit]", self.conf.name);
            return Ok(());
        }

        self.repository.make_tmp_save_commit(message)
    }

    pub fn make_checkpoint_commit(&self, message: Option<&str>) -> ColEyre {
        if !self.repository.is_dirty()? {
            println!("[ Crate `{}` is clean. Skipping commit]", self.conf.name);
            return Ok(());
        }

        self.repository.make_checkpoint_commit(message)
    }

    pub fn make_full_commit(&self, message: &str) -> ColEyre {
        if !self.repository.is_dirty()? {
            println!("[ Crate `{}` is clean. Skipping commit]", self.conf.name);
            return Ok(());
        }

        println!("\n === Running Formater ===\n");
        self.fmt()?;

        if self.conf.sqlx {
            println!("\n === Running sqlx prepare ===\n");
            self.sqlx_prepare()?;
        }

        println!("\n === Running Clippy ===\n");
        self.clippy_hack()?;

        println!("\n === Creating commit ===\n");
        self.repository.make_full_commit(message)
    }
}
