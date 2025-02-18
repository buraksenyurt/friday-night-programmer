use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Tools command line arguments
#[derive(Parser)]
#[command(name = "C# Interface generator")]
#[command(version = "1.0.0")]
#[command(about = "This tool extract interface from class definition")]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

/// User options
#[derive(Subcommand)]
pub enum Commands {
    /// Full class files in a directory
    #[command(aliases = ["d", "dir"])]
    Directory {
        #[arg(value_name = "directory")]
        dir: PathBuf,
    },
}
