use crate::error::{DejaVuError, Result};
use crate::models::file_info::FileInfo;
use crate::scanner::media_filter::MediaFilter;
use std::path::Path;
use walkdir::{WalkDir, DirEntry};

/// File collector for scanning directories and collecting media files
///
/// This struct handles recursive directory traversal and filtering of media files
/// based on file type and minimum size requirements.
pub struct FileCollector {
    filter: MediaFilter,
    min_size: u64,
}

impl FileCollector {
    /// Create a new FileCollector with the specified filter and minimum file size
    pub fn new(filter: MediaFilter, min_size: u64) -> Self {
        Self { filter, min_size }
    }

    /// Collect all media files from the specified directory without progress reporting
    pub fn collect(&self, directory: &Path) -> Result<Vec<FileInfo>> {
        // Create a dummy progress closure that does nothing
        let no_progress = |_found: usize, _total: usize| {};
        self.collect_internal(directory, Some(no_progress))
    }

    /// Collect all media files from the specified directory with progress reporting
    ///
    /// # Arguments
    /// * `directory` - The directory to scan
    /// * `progress` - Callback function that receives (found_count, total_scanned) periodically
    pub fn collect_with_progress<F>(&self, directory: &Path, progress: F) -> Result<Vec<FileInfo>>
    where
        F: FnMut(usize, usize),
    {
        self.collect_internal(directory, Some(progress))
    }

    /// Internal implementation shared by both collect methods
    fn collect_internal<F>(&self, directory: &Path, mut progress: Option<F>) -> Result<Vec<FileInfo>>
    where
        F: FnMut(usize, usize),
    {
        if !directory.exists() {
            return Err(DejaVuError::PathNotFound(
                directory.display().to_string(),
            ));
        }

        let mut files = Vec::new();
        let mut total_scanned = 0;

        for entry in WalkDir::new(directory)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            total_scanned += 1;

            // Update progress if callback provided (every 100 files)
            if let Some(ref mut prog) = progress {
                if total_scanned % 100 == 0 {
                    prog(files.len(), total_scanned);
                }
            }

            // Try to process the entry
            if let Some(file_info) = self.process_entry(&entry) {
                files.push(file_info);
            }
        }

        // Final progress update
        if let Some(mut prog) = progress {
            prog(files.len(), total_scanned);
        }

        Ok(files)
    }

    /// Process a single directory entry and return FileInfo if it's a valid media file
    fn process_entry(&self, entry: &DirEntry) -> Option<FileInfo> {
        let path = entry.path();

        // Skip directories
        if path.is_dir() {
            return None;
        }

        // Check if it's a media file
        if !self.filter.is_media_file(path) {
            return None;
        }

        // Get metadata
        let metadata = std::fs::metadata(path).ok()?;

        // Check file size
        if metadata.len() < self.min_size {
            return None;
        }

        // Get media type
        let media_type = self.filter.get_media_type(path)?;

        // Create FileInfo, handle timestamp errors gracefully
        let modified = metadata.modified().ok()?;

        Some(FileInfo::new(
            path.to_path_buf(),
            metadata.len(),
            modified,
            media_type,
        ))
    }
}
