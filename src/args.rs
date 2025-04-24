use clap::{Parser, Subcommand};

/// Smart file wrangler for the terminal
#[derive(Parser)]
#[command(author, version, about)]
pub struct YeeArgs {
    /// Source directory to scan
    #[arg(short, long, default_value = "./", global = true)]
    pub source_dir: String,

    /// Destination directory to move files to
    #[arg(short, long, default_value = "./out", global = true)]
    pub destination_dir: String,

    /// File pattern to match (glob format)
    #[arg(short, long, default_value = "*", global = true)]
    pub pattern: String,

    /// The operation to perform
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Scan files in source directory matching pattern
    Scan,
    
    /// Move files from source to destination
    Move,
    
    /// Scan and immediately move files
    All,
}

impl YeeArgs {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
