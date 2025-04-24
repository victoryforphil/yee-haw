use std::collections::HashMap;
use log::{debug, info, trace};
use crate::yee_file::YeeFile;

/// Final stage that stores files and detects duplicates based on hash
/// As mentioned in README.md, stores hashes of the files to detect duplicates
pub struct Store {
    /// Original files (non-duplicates)
    originals: Vec<YeeFile>,
    /// Duplicate files
    duplicates: Vec<YeeFile>,
    /// HashMap to track file hashes for faster duplicate detection
    hash_map: HashMap<String, usize>,
}

impl Store {
    /// Create a new empty store
    pub fn new() -> Self {
        Self {
            originals: Vec::new(),
            duplicates: Vec::new(),
            hash_map: HashMap::new(),
        }
    }

    /// Insert a file into the store, detecting duplicates by hash
    /// Returns true if the file was a duplicate, false otherwise
    pub fn insert(&mut self, file: YeeFile) -> bool {
        // Check if the file has a hash
        if let Some(hash) = &file.hash {
            // Check if we've seen this hash before
            if let Some(&original_index) = self.hash_map.get(hash) {
                // This is a duplicate
                debug!(
                    "Found duplicate: {}.{} (hash: {})",
                    file.filename, file.extension, hash
                );
                let original = &self.originals[original_index];
                debug!(
                    "Original is: {}.{} in group {}",
                    original.filename, original.extension, original.group_id
                );
                
                self.duplicates.push(file);
                return true;
            } else {
                // This is a new file
                trace!(
                    "New file: {}.{} (hash: {})",
                    file.filename, file.extension, hash
                );
                let index = self.originals.len();
                self.hash_map.insert(hash.clone(), index);
                self.originals.push(file);
                return false;
            }
        } else {
            // No hash, treat as original
            debug!(
                "No hash for file: {}.{}, treating as original",
                file.filename, file.extension
            );
            self.originals.push(file);
            return false;
        }
    }

    /// Insert multiple files into the store
    pub fn insert_batch(&mut self, files: Vec<YeeFile>) {
        let file_count = files.len();
        info!("Processing batch of {} files", file_count);
        
        let mut duplicate_count = 0;
        for file in files {
            if self.insert(file) {
                duplicate_count += 1;
            }
        }
        
        info!(
            "Batch processing complete. {} originals, {} duplicates",
            file_count - duplicate_count, duplicate_count
        );
    }

    /// Get a reference to the original files
    pub fn originals(&self) -> &Vec<YeeFile> {
        &self.originals
    }

    /// Get a reference to the duplicate files
    pub fn duplicates(&self) -> &Vec<YeeFile> {
        &self.duplicates
    }
    
    /// Count of original files
    pub fn original_count(&self) -> usize {
        self.originals.len()
    }
    
    /// Count of duplicate files
    pub fn duplicate_count(&self) -> usize {
        self.duplicates.len()
    }
    
    /// Total count of all files
    pub fn total_count(&self) -> usize {
        self.originals.len() + self.duplicates.len()
    }
} 