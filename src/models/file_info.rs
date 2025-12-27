//! File information models
//!
//! This module defines data structures for representing media files and their metadata.

use std::path::PathBuf;
use std::time::SystemTime;

/// Media type classification for supported files
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaType {
    /// Image file with specific format
    Image(ImageFormat),
    /// Video file with specific format
    Video(VideoFormat),
}

/// Supported image file formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Jpeg,
    Png,
    Gif,
    Webp,
    Bmp,
    Tiff,
}

/// Supported video file formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoFormat {
    Mp4,
    Mov,
    Avi,
    Mkv,
    Webm,
}

/// Information about a media file
///
/// This struct stores metadata about a media file including its path, size,
/// modification time, and type-specific information like dimensions or duration.
#[derive(Debug, Clone)]
pub struct FileInfo {
    /// Full path to the file
    pub path: PathBuf,
    /// File size in bytes
    pub size: u64,
    /// Last modification timestamp
    pub modified: SystemTime,
    /// Media type classification
    pub file_type: MediaType,
    /// Image dimensions (width, height) if applicable
    pub dimensions: Option<(u32, u32)>,
    /// Video duration in seconds (for videos only)
    pub duration: Option<u64>,
}

impl FileInfo {
    /// Create a new FileInfo instance
    ///
    /// # Arguments
    /// * `path` - Full path to the file
    /// * `size` - File size in bytes
    /// * `modified` - Last modification timestamp
    /// * `file_type` - Media type classification
    pub fn new(
        path: PathBuf,
        size: u64,
        modified: SystemTime,
        file_type: MediaType,
    ) -> Self {
        Self {
            path,
            size,
            modified,
            file_type,
            dimensions: None,
            duration: None,
        }
    }

    /// Get the file name (without path)
    ///
    /// Returns "Unknown" if the file name cannot be extracted as valid UTF-8.
    pub fn filename(&self) -> &str {
        self.path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
    }

    /// Get the file extension
    ///
    /// Returns an empty string if the file has no extension.
    pub fn extension(&self) -> &str {
        self.path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
    }

    /// Check if this file is an image
    pub fn is_image(&self) -> bool {
        matches!(self.file_type, MediaType::Image(_))
    }

    /// Check if this file is a video
    pub fn is_video(&self) -> bool {
        matches!(self.file_type, MediaType::Video(_))
    }
}

impl std::fmt::Display for FileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.filename(), self.size)
    }
}
