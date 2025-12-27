//! Media file filtering
//!
//! This module provides functionality to filter and classify media files based on their extensions.

use crate::models::file_info::{ImageFormat, MediaType, VideoFormat};
use std::path::Path;

/// Filter for identifying and classifying media files
///
/// MediaFilter determines whether a file is a media file (image or video)
/// based on its extension and can parse the extension into a specific MediaType.
pub struct MediaFilter {
    /// Whether to include image files
    images_enabled: bool,
    /// Whether to include video files
    videos_enabled: bool,
}

impl MediaFilter {
    /// Create a new MediaFilter with specified media type preferences
    ///
    /// # Arguments
    /// * `images_enabled` - If true, include image files in filtering
    /// * `videos_enabled` - If true, include video files in filtering
    pub fn new(images_enabled: bool, videos_enabled: bool) -> Self {
        Self {
            images_enabled,
            videos_enabled,
        }
    }

    /// Create a filter that accepts all media types (images and videos)
    pub fn all() -> Self {
        Self {
            images_enabled: true,
            videos_enabled: true,
        }
    }

    /// Check if a path points to a supported media file
    ///
    /// This checks the file extension against known image and video formats.
    ///
    /// # Arguments
    /// * `path` - Path to the file to check
    ///
    /// # Returns
    /// * `true` if the file has a supported media extension and the type is enabled
    /// * `false` otherwise
    pub fn is_media_file(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            let ext_lower = ext.to_lowercase();
            self.images_enabled && self.is_image_ext(&ext_lower)
                || self.videos_enabled && self.is_video_ext(&ext_lower)
        } else {
            false
        }
    }

    /// Get the MediaType for a file path
    ///
    /// Parses the file extension and returns the corresponding MediaType
    /// if it's a supported media format.
    ///
    /// # Arguments
    /// * `path` - Path to the file
    ///
    /// # Returns
    /// * `Some(MediaType)` if the file has a supported extension
    /// * `None` if the extension is not recognized or not enabled
    pub fn get_media_type(&self, path: &Path) -> Option<MediaType> {
        let ext = path.extension()?.to_str()?.to_lowercase();

        if self.images_enabled {
            if let Some(format) = self.parse_image_format(&ext) {
                return Some(MediaType::Image(format));
            }
        }

        if self.videos_enabled {
            if let Some(format) = self.parse_video_format(&ext) {
                return Some(MediaType::Video(format));
            }
        }

        None
    }

    /// Check if an extension is a supported image format
    fn is_image_ext(&self, ext: &str) -> bool {
        matches!(
            ext,
            "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "tif" | "tiff"
        )
    }

    /// Check if an extension is a supported video format
    fn is_video_ext(&self, ext: &str) -> bool {
        matches!(ext, "mp4" | "mov" | "avi" | "mkv" | "webm")
    }

    /// Parse an image extension into its ImageFormat enum
    fn parse_image_format(&self, ext: &str) -> Option<ImageFormat> {
        match ext {
            "jpg" | "jpeg" => Some(ImageFormat::Jpeg),
            "png" => Some(ImageFormat::Png),
            "gif" => Some(ImageFormat::Gif),
            "webp" => Some(ImageFormat::Webp),
            "bmp" => Some(ImageFormat::Bmp),
            "tif" | "tiff" => Some(ImageFormat::Tiff),
            _ => None,
        }
    }

    /// Parse a video extension into its VideoFormat enum
    fn parse_video_format(&self, ext: &str) -> Option<VideoFormat> {
        match ext {
            "mp4" => Some(VideoFormat::Mp4),
            "mov" => Some(VideoFormat::Mov),
            "avi" => Some(VideoFormat::Avi),
            "mkv" => Some(VideoFormat::Mkv),
            "webm" => Some(VideoFormat::Webm),
            _ => None,
        }
    }
}
