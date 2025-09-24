use clap::Parser;

use crate::models::cli_data::CLI_DATA;

/// Initialise the folder using the data from the berger file. This needs to be rerun every major changes in the `berger.toml` file
#[derive(Parser, Debug, Clone)]
pub struct InitCommand;

impl InitCommand {
    pub fn run(&self) -> crate::ColEyre {
        let mut data = CLI_DATA.write().unwrap();

        // We force the init using the auto init flag
        data.set_auto_init(true);

        let _berger = data.get_berger_data()?;

        println!("cargo-berger was successfully initialized!");

        Ok(())
    }
}
