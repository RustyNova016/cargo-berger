use clap::Parser as _;

use crate::cli::Cli;

mod cli;
pub mod models;
pub mod utils;

pub(crate) type ColEyreVal<T> = color_eyre::Result<T>;
pub(crate) type ColEyre<T = ()> = color_eyre::Result<T>;

fn main() -> ColEyre {
    color_eyre::install()?;

    let cli = Cli::parse();
    cli.run()?;

    Ok(())
}
