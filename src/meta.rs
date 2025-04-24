use std::fs::{self, File};
use std::io::{BufReader, BufRead};
use std::path::{Path, PathBuf};
use xxhash_rust::xxh3::xxh3_64;
use log::{debug, trace, info};
use crate::yee_file::YeeFile;
use crate::args::{YeeArgs, RenameStyle, GroupStyle};
use std::collections::HashMap;

/// 2nd stage in our file copier. Will take the list of files from the scanner and add
/// any additional metadata to the files. This includes the hash and destination paths.
pub struct Meta {
    args: YeeArgs,
    // Track group counts for incremental group naming
    group_counters: HashMap<String, usize>,
    // Track file counts for incremental file naming
    file_counters: HashMap<String, usize>,
}

impl Meta {
    pub fn new(args: YeeArgs) -> Self {
        Self {
            args,
            group_counters: HashMap::new(),
            file_counters: HashMap::new(),
        }
    }
    
    /// Process a list of YeeFiles, adding metadata (hash and destination paths) to each file
    pub fn process(&mut self, files: &mut Vec<YeeFile>) -> anyhow::Result<()> {
        debug!("Processing {} files to add metadata", files.len());
        
        // First pass: calculate hashes
        for file in files.iter_mut() {
            let full_path = format!("{}/{}.{}", file.source_full_path, file.filename, file.extension);
            let path = Path::new(&full_path);
            
            match self.hash_file(path) {
                Ok(hash) => {
                    trace!("Added hash {} to file {}.{}", &hash, file.filename, file.extension);
                    file.hash = Some(hash);
                },
                Err(e) => {
                    debug!("Failed to calculate hash for {}: {}", full_path, e);
                }
            }
        }
        
        // Second pass: create destination paths
        for file in files.iter_mut() {
            self.set_destination_paths(file)?;
        }
        
        Ok(())
    }
    
    /// Set destination paths for a file based on args settings
    fn set_destination_paths(&mut self, file: &mut YeeFile) -> anyhow::Result<()> {
        // Create the group folder name based on the selected group style
        let group_folder = self.get_group_folder_name(&file.group_id);
        
        // Create the destination filename based on the selected rename style
        let dest_filename = self.get_destination_filename(file);
        
        // Create the full destination path
        let dest_path = PathBuf::from(&self.args.destination_dir)
            .join(&group_folder);
        
        // Create the directory structure if it doesn't exist
        fs::create_dir_all(&dest_path)?;
        
        // Set the destination paths in the YeeFile
        // destination_full_path should only contain the directory path, not the filename
        file.destination_full_path = dest_path.to_string_lossy().to_string();
        file.destination_local_path = group_folder;
        file.filename = dest_filename;
        
        debug!("Set destination for {}.{}: {}", 
            file.filename, file.extension, 
            file.destination_full_path);
        
        Ok(())
    }
    
    /// Get the group folder name based on the group style
    fn get_group_folder_name(&mut self, group_id: &str) -> String {
        match self.args.group_style {
            GroupStyle::ShortHash => {
                // Use the existing group_id directly
                group_id.to_string()
            },
            GroupStyle::Incremental => {
                // Check if we've seen this group before
                if !self.group_counters.contains_key(group_id) {
                    // First time we've seen this group, assign it the next number
                    let next_counter = self.group_counters.len() + 1;
                    self.group_counters.insert(group_id.to_string(), next_counter);
                }
                
                // Now get the counter value
                let counter = self.group_counters.get(group_id).unwrap_or(&0);
                format!("group_{:04}", counter)
            }
        }
    }
    
    /// Get the destination filename based on the rename style
    fn get_destination_filename(&mut self, file: &YeeFile) -> String {
        match self.args.rename_style {
            RenameStyle::None => {
                // Keep the original filename
                file.filename.clone()
            },
            RenameStyle::Lowercase => {
                // Convert to lowercase
                file.filename.to_lowercase()
            },
            RenameStyle::Incremental => {
                // Use group_id as key to keep related files incrementally numbered together
                let counter = self.file_counters
                    .entry(file.group_id.clone())
                    .or_insert(0);
                *counter += 1;
                
                // Just use the incremental number as the filename
                format!("{:04}", counter)
            },
            RenameStyle::ShortHash => {
                // Use file hash if available, otherwise fallback to original name
                if let Some(hash) = &file.hash {
                    hash[0..8].to_string()
                } else {
                    file.filename.clone()
                }
            },
            RenameStyle::Combined => {
                // Combine group style with filename based on group style
                match self.args.group_style {
                    GroupStyle::ShortHash => {
                        // Use short hash for group and file hash or incremental for file
                        if let Some(hash) = &file.hash {
                            format!("{}_{}", &file.group_id[0..6], &hash[0..8])
                        } else {
                            let counter = self.file_counters
                                .entry(file.group_id.clone())
                                .or_insert(0);
                            *counter += 1;
                            format!("{}_{:04}", &file.group_id[0..6], counter)
                        }
                    },
                    GroupStyle::Incremental => {
                        // Use incremental group number with incremental file number
                        let counter = self.file_counters
                            .entry(file.group_id.clone())
                            .or_insert(0);
                        *counter += 1;
                        
                        let group_num = self.group_counters
                            .get(&file.group_id)
                            .map_or(0, |&num| num);
                            
                        format!("{:03}_{:04}", group_num, counter)
                    }
                }
            }
        }
    }
    
    /// Hash a file using xxHash algorithm (non-cryptographic, very fast)
    fn hash_file(&self, path: &Path) -> anyhow::Result<String> {
        // Open the file and create a buffered reader
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut hasher = xxh3_64(b"");
        
        // Read the file in chunks and update the hash
        loop {
            let buf = reader.fill_buf()?;
            let buf_len = buf.len();
            if buf_len == 0 {
                break;
            }
            
            // Update the hash with this chunk
            hasher = xxh3_64_with_seed(buf, hasher);
            
            // Move the reader's cursor
            reader.consume(buf_len);
        }
        
        // Convert the hash to a string
        Ok(format!("{:016x}", hasher))
    }
}

/// Helper function to incrementally update an xxHash
#[inline]
fn xxh3_64_with_seed(data: &[u8], seed: u64) -> u64 {
    let mut bytes = seed.to_le_bytes().to_vec();
    bytes.extend_from_slice(data);
    xxh3_64(&bytes)
}
