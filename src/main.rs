mod args;
mod scanner;
mod mover;
mod yee_file;
mod meta;

use scanner::Scanner;
use mover::Mover;
use args::{YeeArgs, Commands};
use log::{info, debug};

fn main() {
    pretty_env_logger::init();
    // Parse command line arguments
    let args = YeeArgs::parse_args();
    
    // Create scanner and mover
    let scanner = Scanner::new();
    let mover = Mover::new();
    
    // Process based on command
    match args.command {
        Some(Commands::Scan) => {
            // Only scan files
            let files = scanner.scan(&args.source_dir, &args.pattern);
            info!("Found {} files matching pattern '{}':", files.len(), args.pattern);
            for file in &files {
                debug!("File: {}.{} in group: {}", file.filename, file.extension, file.group_id);
            }
        },
        Some(Commands::Move) => {
            // Scan and then move files
            let files = scanner.scan(&args.source_dir, &args.pattern);
            info!("Moving {} files to {}", files.len(), args.destination_dir);
            mover.move_files(files);
        },
        Some(Commands::All) => {
            // Scan and then move files
            let files = scanner.scan(&args.source_dir, &args.pattern);
            info!("Found {} files. Moving to {}", files.len(), args.destination_dir);
            mover.move_files(files);
        },
        None => {
            // Default behavior - just scan
            let files = scanner.scan(&args.source_dir, &args.pattern);
            info!("Found {} files matching pattern '{}':", files.len(), args.pattern);
            for file in &files {
                debug!("File: {}.{} in group: {}", file.filename, file.extension, file.group_id);
            }
        }
    }
}
