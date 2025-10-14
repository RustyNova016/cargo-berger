use clap::Parser;
use itertools::Itertools as _;

use crate::ColEyre;
use crate::infoln;
use crate::models::berger_data::BergerData;
use crate::models::cli_data::CLI_DATA;

/// Run the CI for the berger workspace.
///
/// This will deactivate any rust workspace, run the designated commands for each crate invidually, then reactivate the workspace.
#[derive(Parser, Debug, Clone)]
pub struct CICommand;

impl CICommand {
    pub fn run(&self) -> crate::ColEyre {
        let berger = CLI_DATA.write().unwrap().get_berger_data()?;

        berger
            .get_rust_workspace(false)?
            .map(|wp| wp.turn_off())
            .transpose()?;

        // We contain the error to first reactivate the workspace before crashing
        let res = Self::process(&berger);

        berger
            .get_rust_workspace(false)?
            .map(|wp| wp.turn_on(&berger.repo_data.values().collect_vec()))
            .transpose()?;

        res
    }

    fn process(berger: &BergerData) -> ColEyre {
        for repo_data in berger.repo_data.values() {
            infoln!("Processing repository `{}`", repo_data.name);

            let Some(crat) = &repo_data.rust else {
                continue;
            };

            crat.run_ci()?;
        }

        Ok(())
    }
}
