use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "nitro")]
#[command(version)]
#[command(about = "Nitro OS package manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(visible_alias = "i")]
    Install {
        packages: Vec<String>,
    },

    #[command(visible_alias = "rm")]
    Remove {
        packages: Vec<String>,
    },

    #[command(visible_alias = "s")]
    Search {
        query: String,
    },

    #[command(visible_alias = "upd")]
    Update,

    #[command(visible_alias = "upg")]
    Upgrade {
        packages: Vec<String>,
    },

    #[command(visible_alias = "cls")]
    Clean,

    #[command(visible_alias = "ls")]
    List,
}
