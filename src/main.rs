mod cli;
mod commands;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { packages } => {
            commands::install::execute(packages)?;
        }

        Commands::Remove { packages } => {
            commands::remove::execute(packages)?;
        }

        Commands::Search { query } => {
            commands::search::execute(query)?;
        }

        Commands::Update => {
            commands::update::execute()?;
        }

        Commands::Upgrade { packages } => {
            commands::upgrade::execute(packages)?;
        }

        Commands::Clean => {
            commands::clean::execute()?;
        }

        Commands::List => {
            commands::list::execute()?;
        }
    }

    Ok(())
}
