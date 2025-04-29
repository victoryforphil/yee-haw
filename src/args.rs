use clap::{Parser, ValueEnum};

/// Smart file wrangler for the terminal
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct YeeArgs {
    /// Source directory to scan
    #[arg(short = 's', long, default_value = "./")]
    pub source_dir: String,

    /// Query (glob pattern) to match files
    #[arg(short = 'q', long, default_value = "*")]
    pub query: String,

    /// Destination directory to move files to
    #[arg(short = 'd', long, default_value = "./out")]
    pub destination_dir: String,

    /// Perform a dry run (don't actually move files)
    #[arg(long)]
    pub dry: bool,

    /// Track and handle duplicates separately
    #[arg(long, default_value_t = true)]
    pub track_duplicates: bool,

    /// File renaming style for destination
    #[arg(long, value_enum, default_value_t = RenameStyle::None)]
    pub rename_style: RenameStyle,

    /// Grouping style for destination folders
    #[arg(long, value_enum, default_value_t = GroupStyle::ShortHash)]
    pub group_style: GroupStyle,
    
    /// Copy files instead of moving them
    #[arg(short = 'c', long, default_value_t = false)]
    pub copy_mode: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum RenameStyle {
    /// Keep original filenames
    None,
    /// Convert filenames to lowercase
    Lowercase,
    /// Add incremental numbers to filenames
    Incremental,
    /// Use short hash for filenames
    ShortHash,
    /// Combine original name with a hash
    Combined,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum GroupStyle {
    /// Use short hash for destination folder names
    ShortHash,
    /// Use incremental numbers for destination folder names
    Incremental,
}

impl YeeArgs {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
