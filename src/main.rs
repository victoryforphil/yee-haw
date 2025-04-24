mod args;
mod scanner;
mod mover;
mod yee_file;
mod meta;

use scanner::Scanner;
use mover::Mover;
use meta::Meta;
use args::{YeeArgs, Commands};
use log::{info, debug, error};

fn main() {
    pretty_env_logger::init();
    // Parse command line arguments
    let args = YeeArgs::parse_args();
    
    // Create components
    let scanner = Scanner::new();
    let meta = Meta::new();
    let mover = Mover::new();
    
    // Process based on command
    match args.command {
        Some(Commands::Scan) => {
            // Only scan files
            let mut files = scanner.scan(&args.source_dir, &args.pattern);
            info!("Found {} files matching pattern '{}':", files.len(), args.pattern);
            
            // Add metadata including hashes
            if let Err(e) = meta.process(&mut files) {
                error!("Error adding metadata: {}", e);
            }
            
            for file in &files {
                debug!("File: {}.{} in group: {} hash: {}", 
                    file.filename, 
                    file.extension, 
                    file.group_id,
                    file.hash.as_deref().unwrap_or("none"));
            }
        },
        Some(Commands::Move) => {
            // Scan and then move files
            let mut files = scanner.scan(&args.source_dir, &args.pattern);
            info!("Found {} files. Adding metadata...", files.len());
            
            // Add metadata including hashes
            if let Err(e) = meta.process(&mut files) {
                error!("Error adding metadata: {}", e);
            }
            
            info!("Moving {} files to {}", files.len(), args.destination_dir);
            mover.move_files(files);
        },
        Some(Commands::All) => {
            // Scan and then move files
            let mut files = scanner.scan(&args.source_dir, &args.pattern);
            info!("Found {} files. Adding metadata...", files.len());
            
            // Add metadata including hashes
            if let Err(e) = meta.process(&mut files) {
                error!("Error adding metadata: {}", e);
            }
            
            info!("Moving {} files to {}", files.len(), args.destination_dir);
            mover.move_files(files);
        },
        None => {
            // Default behavior - just scan
            let mut files = scanner.scan(&args.source_dir, &args.pattern);
            info!("Found {} files matching pattern '{}':", files.len(), args.pattern);
            
            // Add metadata including hashes
            if let Err(e) = meta.process(&mut files) {
                error!("Error adding metadata: {}", e);
            }
            
            for file in &files {
                debug!("File: {}.{} in group: {} hash: {}", 
                    file.filename, 
                    file.extension, 
                    file.group_id,
                    file.hash.as_deref().unwrap_or("none"));
            }
        }
    }
}
