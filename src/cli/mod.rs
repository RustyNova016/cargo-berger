pub mod checkpoint;
pub mod full;
pub mod quick_switch;
pub mod save;
use clap::Parser;
use clap::Subcommand;

use crate::cli::checkpoint::CheckpointCommand;
use crate::cli::full::FullCommand;
use crate::cli::quick_switch::QuickSwitchCommand;
use crate::cli::save::SaveCommand;

/// Tools for TagStudio
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(long, hide = true)]
    pub markdown_help: bool,

    // #[command(flatten)]
    // pub verbose: Verbosity<InfoLevel>,

    // If provided, outputs the completion file for given shell
    // #[arg(long = "generate", value_enum)]
    // generator: Option<Shell>,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

impl Cli {
    pub fn run(&self) -> crate::ColEyre {
        // Invoked as: `$ my-app --markdown-help`
        if self.markdown_help {
            clap_markdown::print_help_markdown::<Self>();
            return Ok(());
        }

        // if let Some(generator) = self.generator {
        //     let mut cmd = Self::command();
        //     Self::print_completions(generator, &mut cmd);
        //     return Ok(false);
        // }

        if let Some(command) = &self.command {
            command.run()?;
        }

        Ok(())
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Checkpoint(CheckpointCommand),
    Full(FullCommand),
    QuickSwitch(QuickSwitchCommand),
    Save(SaveCommand),
}

impl Commands {
    pub fn run(&self) -> crate::ColEyre {
        match self {
            Self::Checkpoint(val) => val.run()?,
            Self::Full(val) => val.run()?,
            Self::QuickSwitch(val) => val.run()?,
            Self::Save(val) => val.run()?,
        }

        Ok(())
    }
}
