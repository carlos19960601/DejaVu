//! Hash-based duplicate detection and grouping
//!
//! This module provides functionality to group duplicate files using various hashing methods.

use crate::error::Result;
use crate::models::file_info::FileInfo;
use crate::models::DuplicateGroup;
use crate::hashing::{ExactHasher, PerceptualHasher};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use indicatif::ProgressBar;
use rayon::prelude::*;

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

    /// Group files by exact SHA-256 hash using multi-threading
    ///
    /// This method computes SHA-256 hashes for all files in parallel using rayon,
    /// then groups files with identical hashes together. Only groups with 2 or more
    /// files are returned.
    ///
    /// # Arguments
    /// * `files` - Vector of files to group
    /// * `progress` - Optional progress bar for status updates
    ///
    /// # Returns
    /// Vector of DuplicateGroup containing only groups with duplicates
    ///
    /// # Performance
    /// Uses multiple CPU cores to compute hashes in parallel, significantly
    /// reducing processing time for large file collections.
    pub fn group_by_exact_hash(&self, files: Vec<FileInfo>, progress: Option<&ProgressBar>) -> Result<Vec<DuplicateGroup>> {
        use std::sync::Mutex;

        let hash_map: HashMap<Vec<u8>, Vec<FileInfo>> = HashMap::new();
        let hash_map = Arc::new(Mutex::new(hash_map));
        let counter = Arc::new(AtomicUsize::new(0));

        // Process files in parallel
        files.par_iter().for_each(|file| {
            // Compute hash for this file
            if let Ok(hash) = ExactHasher::compute_hash(&file.path) {
                // Insert into hash map
                if let Ok(mut map) = hash_map.lock() {
                    map.entry(hash).or_default().push(file.clone());
                }
            }

            // Update progress
            let count = counter.fetch_add(1, Ordering::Relaxed);
            if let Some(pb) = progress {
                pb.set_message(format!("Hashing: {} ({} / {})",
                    file.filename(),
                    count + 1,
                    files.len()
                ));
                pb.set_position(count as u64 + 1);
            }
        });

        // Extract the collected results
        let hash_map = Arc::try_unwrap(hash_map)
            .expect("Arc should have only one reference left")
            .into_inner()
            .map_err(|e| crate::error::DejaVuError::FileOperationFailed(
                format!("Mutex poisoned: {}", e)
            ))?;

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
    /// it only performs exact duplicate detection using SHA-256 hashes
    /// with multi-threading for improved performance.
    /// Perceptual hashing for similar images can be added as a second stage.
    ///
    /// # Arguments
    /// * `files` - Vector of files to analyze
    /// * `progress` - Optional progress bar for status updates
    ///
    /// # Returns
    /// Vector of DuplicateGroup containing all duplicate groups found
    ///
    /// # Performance
    /// Uses rayon for parallel hash computation, automatically utilizing
    /// all available CPU cores for significant speedup on multi-core systems.
    pub fn find_duplicates(&self, files: Vec<FileInfo>, progress: Option<&ProgressBar>) -> Result<Vec<DuplicateGroup>> {
        // Stage 1: Group by exact hash (multi-threaded)
        let exact_groups = self.group_by_exact_hash(files, progress)?;

        // Stage 2: Find similar images within each group and across groups
        // For now, we just return exact duplicates
        // Perceptual hashing can be added as a second pass

        Ok(exact_groups)
    }
}
