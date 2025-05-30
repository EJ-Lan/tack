use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "tack",
    version = "0.1.0",
    about = "A smart CLI bookmark manager for comamnds and directories",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
/// Add a new command to the store
    Add {
        #[arg(help = "The command to store, in quotes")]
        command: String,

        #[arg(short, long, num_args = 1.., help = "Tags for the command")]
        tags: Vec<String>,
    },

    /// Search saved commands
    Search {
        #[arg(help = "Keyword or tag to search")]
        query: String,
    },

    /// List all saved commands
    List {},

    /// Run a stored command by ID
    Run {
        #[arg(help = "ID of the command to run")]
        id: u32,
    },

    /// Remove a saved command by ID
    Remove {
        #[arg(help = "ID of the command to delete")]
        id: u32,
    },

    /// Edit a command or its tags
    Edit {
        #[arg(help = "ID of the command to edit")]
        id: u32,
    },

    /// Initialize config or data files
    Init {},
}
