//! Hash-based duplicate detection and grouping
//!
//! This module provides functionality to group duplicate files using various hashing methods.

use crate::error::Result;
use crate::models::file_info::FileInfo;
use crate::models::DuplicateGroup;
use crate::hashing::{ExactHasher, PerceptualHasher};
use std::collections::HashMap;
use indicatif::ProgressBar;

/// Groups duplicate files using hash-based algorithms
///
/// HashGrouper provides two-stage duplicate detection:
/// 1. Exact duplicates using SHA-256 hashes
/// 2. Similar images using perceptual hashing (future enhancement)
pub struct HashGrouper {
    /// Maximum Hamming distance for perceptual hash similarity (unused in current implementation)
    similarity_threshold: u32,
}

impl HashGrouper {
    /// Create a new HashGrouper with the specified similarity threshold
    ///
    /// # Arguments
    /// * `similarity_threshold` - Maximum Hamming distance for similar images (lower = stricter)
    pub fn new(similarity_threshold: u32) -> Self {
        Self { similarity_threshold }
    }

    /// Group files by exact SHA-256 hash
    ///
    /// This method computes SHA-256 hashes for all files and groups files
    /// with identical hashes together. Only groups with 2 or more files are returned.
    ///
    /// # Arguments
    /// * `files` - Vector of files to group
    /// * `progress` - Optional progress bar for status updates
    ///
    /// # Returns
    /// Vector of DuplicateGroup containing only groups with duplicates
    pub fn group_by_exact_hash(&self, files: Vec<FileInfo>, progress: Option<&ProgressBar>) -> Result<Vec<DuplicateGroup>> {
        let mut hash_map: HashMap<Vec<u8>, Vec<FileInfo>> = HashMap::new();

        for (index, file) in files.into_iter().enumerate() {
            if let Some(pb) = progress {
                pb.set_message(format!("Hashing: {}", file.filename()));
                pb.set_position(index as u64);
            }

            let hash = ExactHasher::compute_hash(&file.path)?;
            hash_map.entry(hash).or_default().push(file);
        }

        // Filter to only groups with duplicates
        let groups: Vec<DuplicateGroup> = hash_map
            .into_iter()
            .filter(|(_, files)| files.len() > 1)
            .enumerate()
            .map(|(i, (hash, files))| DuplicateGroup::new(i, files).with_exact_hash(hash))
            .collect();

        Ok(groups)
    }

    /// Find similar images using perceptual hashing
    ///
    /// This method computes perceptual hashes for images and groups them
    /// based on Hamming distance. Files with Hamming distance below the
    /// threshold are considered similar.
    ///
    /// # Arguments
    /// * `files` - Vector of files to analyze
    /// * `progress` - Optional progress bar for status updates
    ///
    /// # Returns
    /// Vector of DuplicateGroup containing groups of similar images
    ///
    /// # Note
    /// This method currently uses a simplified perceptual hash implementation.
    /// Video files are assigned a dummy hash and are not grouped.
    pub fn find_similar_images(&self, files: Vec<FileInfo>, progress: Option<&ProgressBar>) -> Result<Vec<DuplicateGroup>> {
        let perceptual_hasher = PerceptualHasher::new();
        let mut perceptual_hashes: Vec<u64> = Vec::with_capacity(files.len());

        // Compute perceptual hashes for all images
        for (index, file) in files.iter().enumerate() {
            if let Some(pb) = progress {
                pb.set_message(format!("Computing perceptual hash: {}", file.filename()));
                pb.set_position(index as u64);
            }

            if file.is_image() {
                let hash = perceptual_hasher.compute_hash(&file.path)?;
                perceptual_hashes.push(hash);
            } else {
                // For videos, use a dummy hash (not supported yet)
                perceptual_hashes.push(0);
            }
        }

        // Group similar images
        let mut groups: Vec<DuplicateGroup> = Vec::new();
        let mut assigned = vec![false; files.len()];

        for i in 0..files.len() {
            if assigned[i] || !files[i].is_image() {
                continue;
            }

            let mut similar_files = vec![files[i].clone()];
            assigned[i] = true;

            // Find all similar images
            for j in (i + 1)..files.len() {
                if assigned[j] || !files[j].is_image() {
                    continue;
                }

                if PerceptualHasher::are_similar(
                    perceptual_hashes[i],
                    perceptual_hashes[j],
                    self.similarity_threshold,
                ) {
                    similar_files.push(files[j].clone());
                    assigned[j] = true;
                }
            }

            // Only add if there are duplicates
            if similar_files.len() > 1 {
                let group = DuplicateGroup::new(groups.len(), similar_files)
                    .with_perceptual_hash(perceptual_hashes[i]);
                groups.push(group);
            }
        }

        Ok(groups)
    }

    /// Two-stage duplicate detection: exact hash + perceptual hash
    ///
    /// This is the main entry point for duplicate detection. Currently,
    /// it only performs exact duplicate detection using SHA-256 hashes.
    /// Perceptual hashing for similar images can be added as a second stage.
    ///
    /// # Arguments
    /// * `files` - Vector of files to analyze
    /// * `progress` - Optional progress bar for status updates
    ///
    /// # Returns
    /// Vector of DuplicateGroup containing all duplicate groups found
    pub fn find_duplicates(&self, files: Vec<FileInfo>, progress: Option<&ProgressBar>) -> Result<Vec<DuplicateGroup>> {
        // Stage 1: Group by exact hash
        let exact_groups = self.group_by_exact_hash(files, progress)?;

        // Stage 2: Find similar images within each group and across groups
        // For now, we just return exact duplicates
        // Perceptual hashing can be added as a second pass

        Ok(exact_groups)
    }
}
