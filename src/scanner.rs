use std::fs;
use std::path::{Path, PathBuf};
use glob::Pattern;
use crate::yee_file::YeeFile;

/// First stage in our file copier. Will scan the provided
/// root directory recursively and return a list of files
/// that match a provided regex / glob pattern.
pub struct Scanner{}

impl Scanner{
 pub fn new() -> Self{
    Self{}
 }

 pub fn scan(&self, root_dir: &str, pattern: &str) -> Vec<YeeFile>{
    let mut files = Vec::new();
    let mut queue = Vec::new();

    let pattern = Pattern::new(pattern).expect("Invalid glob pattern");
    let root_path = Path::new(root_dir);

    queue.push(PathBuf::from(root_dir));
    
    while let Some(dir_path) = queue.pop() {
        if let Ok(entries) = fs::read_dir(&dir_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    
                    if path.is_dir() {
                        queue.push(path);
                    } else if path.is_file() {
                        // Check if the file matches the pattern
                        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                            if pattern.matches(file_name) {
                                if let Some(yee_file) = YeeFile::from_path(root_path, &path) {
                                    files.push(yee_file);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    files
 }
}