mod cli;

use cli::{Cli, Commands};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { command, tags } => {
            println!("Would add command: '{}', with tags: {:?}", command, tags);
        }

        Commands::Search { query } => {
            println!("Would search for: '{}'", query);
        }

        Commands::List {} => {
            println!("Would list all saved commands.");
        }

        Commands::Run { id } => {
            println!("Would run command with ID: {}", id);
        }

        Commands::Remove { id } => {
            println!("Would remove command with ID: {}", id);
        }

        Commands::Edit { id } => {
            println!("Would edit command with ID: {}", id);
        }

        Commands::Init {} => {
            println!("Would initialize data/config files.");
        }
    }
}
