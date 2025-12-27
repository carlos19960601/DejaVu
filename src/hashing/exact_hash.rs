use crate::error::Result;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct ExactHasher;

impl ExactHasher {
    /// Compute SHA-256 hash of a file
    /// Reads file in 64KB chunks to avoid loading entire file into memory
    pub fn compute_hash(path: &Path) -> Result<Vec<u8>> {
        let file = File::open(path)?;
        let mut reader = std::io::BufReader::with_capacity(65536, file);

        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 65536]; // 64KB buffer

        loop {
            let n = reader.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }

        Ok(hasher.finalize().to_vec())
    }

    /// Compute hash as hex string
    pub fn compute_hash_string(path: &Path) -> Result<String> {
        let hash = Self::compute_hash(path)?;
        Ok(hex::encode(hash))
    }

    /// Check if two files have the same hash without storing the full hash
    pub fn files_equal(path1: &Path, path2: &Path) -> Result<bool> {
        let hash1 = Self::compute_hash(path1)?;
        let hash2 = Self::compute_hash(path2)?;
        Ok(hash1 == hash2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_compute_hash() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"Hello, World!").unwrap();

        let hash = ExactHasher::compute_hash(temp_file.path()).unwrap();
        assert_eq!(hash.len(), 32); // SHA-256 produces 32 bytes
    }

    #[test]
    fn test_files_equal() {
        let mut temp_file1 = NamedTempFile::new().unwrap();
        temp_file1.write_all(b"Same content").unwrap();

        let mut temp_file2 = NamedTempFile::new().unwrap();
        temp_file2.write_all(b"Same content").unwrap();

        assert!(ExactHasher::files_equal(temp_file1.path(), temp_file2.path()).unwrap());
    }
}
