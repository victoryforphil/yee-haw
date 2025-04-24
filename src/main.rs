mod args;
mod scanner;
mod mover;
mod yee_file;
mod meta;
mod store;

use scanner::Scanner;
use mover::Mover;
use meta::Meta;
use store::Store;
use args::YeeArgs;
use log::{info, debug, error};

/// Main entry point for the Yee-Haw file organization tool
fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    
    // Parse command line arguments
    let args = YeeArgs::parse_args();
    
    // Create components
    let scanner = Scanner::new();
    let mut meta = Meta::new(args.clone());
    let mover = Mover::new(args.clone());
    
    // === Step 1: Scan for files ===
    info!("Scanning directory '{}' for files matching '{}'", args.source_dir, args.query);
    let mut files = scanner.scan(&args.source_dir, &args.query);
    info!("Found {} files matching pattern", files.len());
    
    if files.is_empty() {
        info!("No files found. Exiting.");
        return Ok(());
    }
    
    // === Step 2: Process metadata (calculate hashes and set destination paths) ===
    info!("Processing file metadata and creating destination paths...");
    if let Err(e) = meta.process(&mut files) {
        error!("Error processing metadata: {}", e);
    }
    
    // === Step 3: Handle duplicates if tracking is enabled ===
    let mut store = Store::new();
    
    if args.track_duplicates {
        info!("Checking for duplicate files...");
        store.insert_batch(files);
        
        info!(
            "Found {} original files and {} duplicates", 
            store.original_count(), 
            store.duplicate_count()
        );
        
        if args.dry {
            // In dry run mode, just show what would happen
            info!(
                "DRY RUN: Would move {} original files to their destination folders", 
                store.original_count()
            );
            
            if store.duplicate_count() > 0 {
                info!(
                    "DRY RUN: Would move {} duplicate files to the _dupes directory", 
                    store.duplicate_count()
                );
            }
            
            debug!("Renaming style: {:?}, Group style: {:?}", 
                args.rename_style, args.group_style);
            
            // Display sample destination paths for a few files
            let sample_size = std::cmp::min(3, store.original_count());
            if sample_size > 0 {
                debug!("Sample destination paths:");
                for (i, file) in store.originals().iter().take(sample_size).enumerate() {
                    debug!(
                        "  {}: {}/{}.{}", 
                        i+1,
                        file.destination_full_path,
                        file.filename,
                        file.extension
                    );
                }
            }
            
            debug!("Duplicate files that would be skipped:");
            for file in store.duplicates() {
                debug!(
                    "  {}.{} (hash: {})", 
                    file.filename,
                    file.extension,
                    file.hash.as_deref().unwrap_or("none")
                );
            }
        } else {
            // Actually move the files
            info!(
                "Moving {} original files to their destination folders", 
                store.original_count()
            );
            
            // Destination paths are already set by the meta processor
            mover.move_files(store.originals().to_vec())?;
            
            // Move duplicates to the _dupes directory
            if store.duplicate_count() > 0 {
                mover.move_duplicates(store.duplicates().to_vec())?;
            }
        }
    } else {
        // No duplicate tracking, just move all files
        info!("Duplicate tracking disabled");
        
        if args.dry {
            info!(
                "DRY RUN: Would move {} files to their destination folders", 
                files.len()
            );
            
            debug!("Renaming style: {:?}, Group style: {:?}", 
                args.rename_style, args.group_style);
                
            // Display sample destination paths for a few files
            let sample_size = std::cmp::min(3, files.len());
            if sample_size > 0 {
                debug!("Sample destination paths:");
                for (i, file) in files.iter().take(sample_size).enumerate() {
                    debug!(
                        "  {}: {}/{}.{}", 
                        i+1,
                        file.destination_full_path,
                        file.filename,
                        file.extension
                    );
                }
            }
        } else {
            info!("Moving {} files to their destination folders", files.len());
            
            // Destination paths are already set by the meta processor
            mover.move_files(files)?;
        }
    }
    
    info!("Operation complete.");
    Ok(())
}
