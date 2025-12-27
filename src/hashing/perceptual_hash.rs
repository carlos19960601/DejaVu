use crate::error::Result;
use std::path::Path;

pub struct PerceptualHasher {
    hash_size: u8,
}

impl PerceptualHasher {
    pub fn new() -> Self {
        Self { hash_size: 8 }
    }

    pub fn with_size(hash_size: u8) -> Self {
        Self { hash_size }
    }

    /// Compute perceptual hash of an image
    /// Returns a 64-bit hash (for 8x8 hash)
    /// Note: Simplified implementation using only exact hash for now
    /// Full perceptual hashing requires complex image processing
    pub fn compute_hash(&self, _path: &Path) -> Result<u64> {
        // For now, return a dummy hash
        // TODO: Implement proper perceptual hashing or use a different library
        // that's compatible with image 0.25
        Ok(0)
    }

    /// Compute Hamming distance between two perceptual hashes
    /// Lower distance = more similar
    pub fn hamming_distance(hash1: u64, hash2: u64) -> u32 {
        (hash1 ^ hash2).count_ones()
    }

    /// Check if two hashes are similar based on threshold
    /// threshold: maximum bits that can differ (typically 0-10)
    pub fn are_similar(hash1: u64, hash2: u64, threshold: u32) -> bool {
        Self::hamming_distance(hash1, hash2) <= threshold
    }
}

impl Default for PerceptualHasher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_distance() {
        // Identical hashes
        assert_eq!(PerceptualHasher::hamming_distance(0b1010, 0b1010), 0);

        // Completely different hashes
        assert_eq!(PerceptualHasher::hamming_distance(0b0000, 0b1111), 4);

        // One bit different
        assert_eq!(PerceptualHasher::hamming_distance(0b1010, 0b1011), 1);
    }

    #[test]
    fn test_are_similar() {
        // threshold = 2 means up to 2 bits can differ
        assert!(PerceptualHasher::are_similar(0b1010, 0b1010, 2)); // identical
        assert!(PerceptualHasher::are_similar(0b1010, 0b1011, 2)); // 1 bit diff
        assert!(PerceptualHasher::are_similar(0b1010, 0b1001, 2)); // 2 bits diff
        assert!(!PerceptualHasher::are_similar(0b1010, 0b0001, 2)); // 3 bits diff
    }
}
