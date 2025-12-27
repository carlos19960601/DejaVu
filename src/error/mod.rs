use thiserror::Error;

#[derive(Error, Debug)]
pub enum DejaVuError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Image processing error: {0}")]
    Image(#[from] image::ImageError),

    #[error("Walkdir error: {0}")]
    Walkdir(#[from] walkdir::Error),

    #[error("Path does not exist: {0}")]
    PathNotFound(String),

    #[error("Invalid file type: {0}")]
    InvalidFileType(String),

    #[error("Hash computation failed: {0}")]
    HashError(String),

    #[error("No duplicate files found")]
    NoDuplicatesFound,

    #[error("File operation failed: {0}")]
    FileOperationFailed(String),
}

pub type Result<T> = std::result::Result<T, DejaVuError>;
