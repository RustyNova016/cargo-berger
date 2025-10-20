use crate::ColEyre;
use crate::models::repository_data::RepositoryData;
use crate::models::repository_data::rust::RustData;

impl RustData {
    pub fn install_git_patches(&self, crates: &[&RepositoryData]) -> ColEyre {
        for crat in crates {
            let Some(rust_data) = &crat.rust else {
                continue;
            };

            let Some(git_repo) = &crat.conf.remote_url else {
                continue;
            };

            if self.cargo_file.get_package_name() == rust_data.cargo_file.get_package_name() {
                continue;
            }

            // Get the latest rev for the crate
            let rev = crat.new_command().git().rev_parse_head()?;

            self.cargo_file.add_git_patch(
                rust_data.cargo_file.get_package_name(),
                git_repo.to_owned(),
                rev.trim().to_owned(),
            );

            self.cargo_file.save()?;
        }

        Ok(())
    }

    pub fn remove_patches(&self) -> ColEyre {
        self.cargo_file.remove_patches();
        self.cargo_file.save()
    }
}
