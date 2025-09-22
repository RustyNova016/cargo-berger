use std::path::Path;

use color_eyre::eyre::ContextCompat;

use crate::ColEyre;
use crate::models::tool_bindings::cargo::cargo_file::CargoFile;

impl CargoFile {
    pub fn add_crate_to_workplace(&mut self, path: &Path) -> ColEyre {
        self.doc["workplace"]["members"]
            .as_array_mut()
            .context(
                "Error while adding crate to workplace: Expected workplace.members to be an array",
            )?
            .push(path.to_string_lossy().to_string());

        Ok(())
    }

    pub fn remove_crate_to_workplace(&mut self, path: &Path) -> ColEyre {
        let path = path.to_string_lossy().to_string();

        self.doc["workplace"]["members"]
            .as_array_mut()
            .context(
                "Error while adding crate to workplace: Expected workplace.members to be an array",
            )?
            .retain(|val| val.as_str().is_none_or(|val| val != path));

        Ok(())
    }
}
