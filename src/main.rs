mod cli;
mod models;

use cli::{Cli, Commands};
use models::CommandEntry;
use uuid::Uuid;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { command, tags } => {
            let entry = CommandEntry {
                id: Uuid::new_v4(),
                command: command.clone(),
                tags: tags.clone(),
                created_at: chrono::Utc::now(),
                last_run: None,
            };
            
            println!("Mock entry created:\n{:#?}", entry);
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
