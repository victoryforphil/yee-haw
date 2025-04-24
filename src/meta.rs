use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use xxhash_rust::xxh3::xxh3_64;
use log::{debug, trace};
use crate::yee_file::YeeFile;

/// 2nd stage in our file copier. Will take the list of files from the scanner and add
/// any additional metadata to the files. This includes the hash.
pub struct Meta{}

impl Meta{
    pub fn new() -> Self{
        Self{}
    }
    
    /// Process a list of YeeFiles, adding hash metadata to each file
    pub fn process(&self, files: &mut Vec<YeeFile>) -> anyhow::Result<()> {
        debug!("Processing {} files to add metadata", files.len());
        
        for file in files {
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
        
        Ok(())
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
