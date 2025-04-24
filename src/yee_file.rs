use std::path::Path;
use tiny_id::ShortCodeGenerator;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use log::{debug, trace};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct YeeFile{
    // Name of file without extension or path 
    pub filename: String,
    pub extension: String,
    // Full path to the file excluding filename + extension
    pub source_full_path: String,

    // Full path to the file excluding filename + extension
    pub destination_full_path: String,

    // Path with the root of the scan removed
    pub source_local_path: String,
    // Path with the root of the storage removed
    pub destination_local_path: String,

    // Hash of the file
    pub hash: Option<String>,

    // Is based off source_local_path as this defines the group. Its hashed and used
    // to generate a short hash.
    pub group_id: String,
}

impl YeeFile {
    pub fn from_path(root_path: &Path, file_path: &Path) -> Option<Self> {
        let file_name = file_path.file_stem()?.to_str()?.to_string();
        let extension = file_path.extension().and_then(|ext| ext.to_str())?.to_string();
        
        let source_full_path = if let Some(parent) = file_path.parent() {
            parent.to_string_lossy().to_string()
        } else {
            debug!("Failed to get parent path for file: {:?}", file_path);
            return None;
        };
        
        // Calculate the local path by removing the root_path
        let source_local_path = if let Ok(rel_path) = file_path.strip_prefix(root_path) {
            if let Some(parent) = rel_path.parent() {
                parent.to_string_lossy().to_string()
            } else {
                String::new()
            }
        } else {
            debug!("Failed to strip prefix from path: {:?}", file_path);
            return None;
        };
        
        // For now, we'll leave destination paths empty as they'll be set by the mover
        let destination_full_path = String::new();
        let destination_local_path = String::new();
        
        // Generate a short ID for the group_id based on the local path
        let group_id = Self::generate_short_id(&source_local_path);
        
        trace!("Created YeeFile: {} with group_id: {}", file_name, group_id);
        
        Some(Self {
            filename: file_name,
            extension,
            source_full_path,
            destination_full_path,
            source_local_path,
            destination_local_path,
            hash: None,
            group_id,
        })
    }

    /// Generate a short ID based on the path string
    fn generate_short_id(path: &str) -> String {
        // Create a hash of the path
        let mut hasher = DefaultHasher::new();
        path.hash(&mut hasher);
        let hash_value = hasher.finish();
        
        // Use the hash to seed a short code generator
        // We'll generate 8-character alphanumeric codes
        let mut generator = ShortCodeGenerator::new_alphanumeric(8);
        
        // Skip forward based on hash value to get a consistent ID for the same path
        for _ in 0..hash_value % 1000 {
            generator.next();
        }
        
        let id = generator.next_string();
        trace!("Generated group_id: {} for path: {}", id, path);
        id
    }
}
