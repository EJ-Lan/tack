use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "tack",
    version = "0.1.0",
    about = "Bookmark commands and paths"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new command or path bookmark
    Add,

    /// List all bookmarks
    List,

    /// Search bookmarks
    Search,

    /// Run a command bookmark
    Run,

    /// Open a path bookmark
    Open,

    /// Edit a bookmark
    Edit,

    /// Remove a bookmark
    Remove,

    /// Prune invalid paths
    Prune,

    /// Initialize tack storage
    Init,
}
