use clap::Parser;

use crate::models::cli_data::CLI_DATA;

/// Initialise the folder using the data from the berger file. This needs to be rerun every major changes in the `berger.toml` file
#[derive(Parser, Debug, Clone)]
pub struct DebugListCommand;

impl DebugListCommand {
    pub fn run(&self) -> crate::ColEyre {
        let berger = CLI_DATA.write().unwrap().get_berger_data()?;

        println!("Repositories: ");
        for repo_data in berger.repo_data.values() {
            println!(
                "   - {} at {}",
                repo_data.name,
                repo_data.get_directory().display()
            )
        }

        Ok(())
    }
}
