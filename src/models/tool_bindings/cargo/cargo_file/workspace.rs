use std::path::Path;

use color_eyre::eyre::ContextCompat;
use color_eyre::eyre::eyre;
use toml_edit::Array;
use toml_edit::DocumentMut;
use toml_edit::Item;
use toml_edit::Table;
use toml_edit::Value;

use crate::ColEyre;
use crate::ColEyreVal;
use crate::models::tool_bindings::cargo::cargo_file::CargoFile;

impl CargoFile {
    pub fn get_mut_workspace(doc: &mut DocumentMut) -> &mut Item {
        doc.entry("workspace")
            .or_insert_with(|| Item::Table(Table::new()))
    }

    pub fn get_mut_workspace_members(doc: &mut DocumentMut) -> ColEyreVal<&mut Array> {
        let workspace = Self::get_mut_workspace(doc);

        let members = &mut workspace["members"];

        if let Item::Value(val) = members {
            return val.as_array_mut().context(
                "Error while adding crate to workspace: Expected workspace.members to be an array",
            );
        }

        if members.is_none() {
            *members = Item::Value(Value::Array(Default::default()));
            return members.as_array_mut().context(
                "Error while adding crate to workspace: Expected workspace.members to be an array",
            );
        }

        Err(eyre!(
            "Error while adding crate to workspace: Expected workspace.members to be an array"
        ))
    }

    pub fn add_crate_to_workspace(&self, path: &Path) -> ColEyre {
        let mut doc = self.doc.lock().unwrap();
        let members = Self::get_mut_workspace_members(&mut doc)?;

        if !members.iter().any(|member| {
            member
                .as_str()
                .is_some_and(|member| member == path.to_string_lossy())
        }) {
            members.push(path.to_string_lossy().to_string());
        }

        Ok(())
    }

    pub fn remove_crate_to_workspace(&self, path: &Path) -> ColEyre {
        let mut doc = self.doc.lock().unwrap();
        let members = Self::get_mut_workspace_members(&mut doc)?;
        let path = path.to_string_lossy().to_string();

        members.retain(|val| val.as_str().is_none_or(|val| val != path));

        Ok(())
    }

    pub fn set_workspace_resolver(&self, version: &str) {
        let mut doc = self.doc.lock().unwrap();
        Self::get_mut_workspace(&mut doc)["resolver"] = Item::Value(Value::from(version))
    }
}
