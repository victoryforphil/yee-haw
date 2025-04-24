use std::fs;
use std::path::Path;
use log::{debug, info, warn};
use crate::yee_file::YeeFile;
use crate::args::YeeArgs;

/// Final stage in the file processing pipeline.
/// Responsible for moving files from their source to the destination.
/// 
/// If duplicate tracking is enabled, duplicates will be moved to a "_dupes" directory
/// within the source directory while maintaining the destination directory structure.
pub struct Mover {
    args: YeeArgs,
}

impl Mover {
    /// Creates a new Mover instance
    pub fn new(args: YeeArgs) -> Self {
        Self { args }
    }

    /// Moves the given files to their destination paths.
    /// 
    /// Each file's destination_full_path should already be set.
    pub fn move_files(&self, files: Vec<YeeFile>) -> anyhow::Result<()> {
        info!("Moving {} files to their destination", files.len());
        
        for file in files {
            self.move_single_file(file)?;
        }
        
        info!("File moving complete");
        Ok(())
    }

    /// Moves duplicate files to the _dupes directory.
    /// 
    /// Duplicates are stored in source_dir/_dupes/ with the same folder structure
    /// as the originals would have in the destination directory.
    pub fn move_duplicates(&self, duplicates: Vec<YeeFile>) -> anyhow::Result<()> {
        if duplicates.is_empty() {
            return Ok(());
        }

        info!("Moving {} duplicate files to dupes directory", duplicates.len());
        
        for file in duplicates {
            self.move_duplicate_file(file)?;
        }
        
        info!("Duplicate file moving complete");
        Ok(())
    }

    /// Moves a single file to its destination path
    fn move_single_file(&self, file: YeeFile) -> anyhow::Result<()> {
        let source_path = format!("{}/{}.{}", file.source_full_path, file.filename, file.extension);
        let destination_path = file.destination_full_path.clone();
        
        debug!("Moving file from {} to {}", source_path, destination_path);
        
        // Ensure the directory exists
        if let Some(parent) = Path::new(&destination_path).parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Copy the file
        match fs::copy(&source_path, &destination_path) {
            Ok(_) => debug!("Successfully copied file to {}", destination_path),
            Err(e) => warn!("Failed to copy file to {}: {}", destination_path, e),
        }

        Ok(())
    }

    /// Moves a duplicate file to the _dupes directory
    fn move_duplicate_file(&self, file: YeeFile) -> anyhow::Result<()> {
        let source_path = format!("{}/{}.{}", file.source_full_path, file.filename, file.extension);
        
        // Create a path for duplicates: source_dir/_dupes/[original_destination_structure]
        let source_root = Path::new(&self.args.source_dir);
        let dupes_dir = source_root.join("_dupes");
        
        // Keep the same destination layout but under the _dupes directory
        let relative_dest_path = if let Ok(rel_path) = Path::new(&file.destination_full_path)
            .strip_prefix(Path::new(&self.args.destination_dir)) {
            rel_path
        } else {
            // Fallback if we can't determine the relative path
            Path::new(&file.destination_local_path)
        };
        
        let dupe_dest_path = dupes_dir.join(relative_dest_path);
        let dupe_dest_path_str = dupe_dest_path.to_string_lossy().to_string();
        
        debug!("Moving duplicate file from {} to {}", source_path, dupe_dest_path_str);
        
        // Ensure the directory exists
        if let Some(parent) = dupe_dest_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Copy the file
        match fs::copy(&source_path, &dupe_dest_path) {
            Ok(_) => debug!("Successfully copied duplicate file to {}", dupe_dest_path_str),
            Err(e) => warn!("Failed to copy duplicate file to {}: {}", dupe_dest_path_str, e),
        }

        Ok(())
    }
}