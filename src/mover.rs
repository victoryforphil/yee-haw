use std::fs;
use std::path::Path;
use log::{debug, info, warn};
use crate::yee_file::YeeFile;
use crate::args::YeeArgs;
use std::collections::HashMap;
use std::io::Write;

/// Final stage in our file processing pipeline. Takes the files that have been 
/// fully processed and moves or copies them to their destination.
/// 
/// Also generates metadata YAML files in the destination's .yeehaw directories.
/// If duplicate tracking is enabled, duplicates will be moved to a "_dupes" directory
/// within the destination directory.
/// 
/// When copy_mode is enabled, files are copied instead of moved, preserving the originals.
pub struct Mover {
    args: YeeArgs,
}

impl Mover {
    /// Creates a new Mover instance
    pub fn new(args: YeeArgs) -> Self {
        Self { args }
    }

    /// Moves or copies the given files to their destination paths based on copy_mode.
    /// 
    /// Each file's destination_full_path should already be set.
    pub fn move_files(&self, files: Vec<YeeFile>) -> anyhow::Result<()> {
        let action = if self.args.copy_mode { "Copying" } else { "Moving" };
        info!("{} {} files to their destination", action, files.len());
        
        // Group files by group_id for metadata tracking
        let mut grouped_files: HashMap<String, Vec<YeeFile>> = HashMap::new();
        
        for file in &files {
            grouped_files
                .entry(file.group_id.clone())
                .or_insert_with(Vec::new)
                .push(file.clone());
        }
        
        // Create metadata for each group
        for (group_id, group_files) in &grouped_files {
            self.write_group_metadata(group_id, group_files)?;
        }
        
        for file in files {
            self.process_single_file(file)?;
        }
        
        let action_complete = if self.args.copy_mode { "File copying" } else { "File moving" };
        info!("{} complete", action_complete);
        Ok(())
    }

    /// Moves or copies duplicate files to the _dupes directory based on copy_mode.
    /// 
    /// Duplicates are stored in destination_dir/_dupes/ with the same structure
    /// as the originals would have in the destination directory.
    pub fn move_duplicates(&self, duplicates: Vec<YeeFile>) -> anyhow::Result<()> {
        if duplicates.is_empty() {
            return Ok(());
        }

        let action = if self.args.copy_mode { "Copying" } else { "Moving" };
        info!("{} {} duplicate files to dupes directory", action, duplicates.len());
        
        // Group duplicate files by group_id for metadata tracking
        let mut grouped_dupes: HashMap<String, Vec<YeeFile>> = HashMap::new();
        
        for file in &duplicates {
            grouped_dupes
                .entry(file.group_id.clone())
                .or_insert_with(Vec::new)
                .push(file.clone());
        }
        
        // Create metadata for duplicate groups
        for (group_id, group_files) in &grouped_dupes {
            self.write_group_metadata(group_id, group_files)?;
        }
        
        for file in duplicates {
            self.process_duplicate_file(file)?;
        }
        
        let action_complete = if self.args.copy_mode { "Duplicate file copying" } else { "Duplicate file moving" };
        info!("{} complete", action_complete);
        Ok(())
    }

    /// Writes metadata for a group of files to a YAML file in the .yeehaw directory
    fn write_group_metadata(&self, group_id: &str, files: &[YeeFile]) -> anyhow::Result<()> {
        // Base path for the destination directory
        let dest_root = Path::new(&self.args.destination_dir);
        
        // Get the path to the group's first file to determine where to store metadata
        if let Some(first_file) = files.first() {
            // Create a .yeehaw directory in the destination directory that contains the group
            let group_path = if first_file.destination_local_path.is_empty() {
                dest_root.to_path_buf()
            } else {
                dest_root.join(&first_file.destination_local_path)
            };
            
            let yeehaw_dir = group_path.join(".yeehaw");
            
            // Create the .yeehaw directory if it doesn't exist
            fs::create_dir_all(&yeehaw_dir)?;
            
            // Create a YAML file for each file's metadata
            for file in files {
                let metadata_filename = format!("{}_{}_{}.yaml", 
                    group_id,
                    file.filename, 
                    file.extension);
                let metadata_path = yeehaw_dir.join(metadata_filename);
                
                // Serialize the YeeFile to YAML
                let yaml_content = serde_yaml::to_string(file)?;
                
                // Write the YAML content to a file
                let mut file = fs::File::create(metadata_path)?;
                file.write_all(yaml_content.as_bytes())?;
            }
            
            // Write a group summary file
            let group_summary_path = yeehaw_dir.join(format!("{}_summary.yaml", group_id));
            
            // Create a summary struct with group info
            #[derive(serde::Serialize)]
            struct GroupSummary {
                group_id: String,
                file_count: usize,
                files: Vec<String>,
            }
            
            let summary = GroupSummary {
                group_id: group_id.to_string(),
                file_count: files.len(),
                files: files.iter()
                    .map(|f| format!("{}.{}", f.filename, f.extension))
                    .collect(),
            };
            
            // Serialize and write the summary
            let summary_yaml = serde_yaml::to_string(&summary)?;
            let mut summary_file = fs::File::create(group_summary_path)?;
            summary_file.write_all(summary_yaml.as_bytes())?;
            
            debug!("Wrote group summary for {} to YAML in {}", group_id, yeehaw_dir.display());
        }
        
        Ok(())
    }

    /// Processes a single file (either copy or move based on copy_mode)
    fn process_single_file(&self, file: YeeFile) -> anyhow::Result<()> {
        let source_path = format!("{}/{}.{}", file.source_full_path, file.filename, file.extension);
        let destination_path = format!("{}/{}.{}", file.destination_full_path, file.filename, file.extension);
        
        let action = if self.args.copy_mode { "Copying" } else { "Moving" };
        debug!("{} file from {} to {}", action, source_path, destination_path);
        
        // Ensure the directory exists
        if let Some(parent) = Path::new(&destination_path).parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Copy the file
        match fs::copy(&source_path, &destination_path) {
            Ok(_) => {
                debug!("Successfully copied file to {}", destination_path);
                
                // If not in copy mode (i.e., move mode), delete the source file
                if !self.args.copy_mode {
                    if let Err(e) = fs::remove_file(&source_path) {
                        warn!("Failed to delete source file {}: {}", source_path, e);
                    } else {
                        debug!("Deleted source file after move: {}", source_path);
                    }
                }
            },
            Err(e) => warn!("Failed to copy file to {}: {}", destination_path, e),
        }

        Ok(())
    }

    /// Processes a duplicate file (either copy or move based on copy_mode)
    /// 
    /// Duplicates are stored in destination_dir/_dupes/ with the same structure
    /// as the originals would have in the destination directory.
    fn process_duplicate_file(&self, file: YeeFile) -> anyhow::Result<()> {
        let source_path = format!("{}/{}.{}", file.source_full_path, file.filename, file.extension);
        
        // Create a path for duplicates: destination_dir/_dupes/[original_destination_structure]
        let dest_root = Path::new(&self.args.destination_dir);
        let dupes_dir = dest_root.join("_dupes");
        
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
        
        let action = if self.args.copy_mode { "Copying" } else { "Moving" };
        debug!("{} duplicate file from {} to {}", action, source_path, dupe_dest_path_str);
        
        // Ensure the directory exists
        if let Some(parent) = dupe_dest_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Copy the file
        match fs::copy(&source_path, &dupe_dest_path) {
            Ok(_) => {
                debug!("Successfully copied duplicate file to {}", dupe_dest_path_str);
                
                // If not in copy mode (i.e., move mode), delete the source file
                if !self.args.copy_mode {
                    if let Err(e) = fs::remove_file(&source_path) {
                        warn!("Failed to delete duplicate source file {}: {}", source_path, e);
                    } else {
                        debug!("Deleted source file after duplicate move: {}", source_path);
                    }
                }
            },
            Err(e) => warn!("Failed to copy duplicate file to {}: {}", dupe_dest_path_str, e),
        }

        Ok(())
    }
}