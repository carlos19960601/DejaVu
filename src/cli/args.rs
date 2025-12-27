use clap::Parser;
use std::path::PathBuf;

/// DejaVu - A TUI duplicate file finder for images and videos
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Directory to scan for duplicate files
    #[arg(value_name = "DIRECTORY")]
    pub directory: PathBuf,

    /// Scan images only
    #[arg(short = 'i', long)]
    pub images_only: bool,

    /// Scan videos only
    #[arg(short = 'v', long)]
    pub videos_only: bool,

    /// Similarity threshold for perceptual hashing (0-10, default: 5)
    #[arg(short = 't', long, default_value = "5")]
    pub threshold: u32,

    /// Minimum file size in bytes (default: 1024)
    #[arg(short = 's', long, default_value = "1024")]
    pub min_size: u64,
}
