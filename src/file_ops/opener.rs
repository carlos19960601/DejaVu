use crate::error::Result;
use std::path::Path;

pub struct FileOpener;

impl FileOpener {
    /// Open a file with the system's default application
    pub fn open(path: &Path) -> Result<()> {
        open::that(path).map_err(|e| {
            crate::error::DejaVuError::FileOperationFailed(format!(
                "Failed to open {}: {}",
                path.display(),
                e
            ))
        })?;
        Ok(())
    }

    /// Show a file in the system's file manager
    #[cfg(target_os = "macos")]
    pub fn reveal(path: &Path) -> Result<()> {
        std::process::Command::new("open")
            .arg("-R")
            .arg(path)
            .spawn()
            .map_err(|e| {
                crate::error::DejaVuError::FileOperationFailed(format!(
                    "Failed to reveal {}: {}",
                    path.display(),
                    e
                ))
            })?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    pub fn reveal(path: &Path) -> Result<()> {
        std::process::Command::new("dbus-send")
            .args([
                "--session",
                "--dest=org.freedesktop.FileManager1",
                "--type=method_call",
                "/org/freedesktop/FileManager1",
                "org.freedesktop.FileManager1.ShowItems",
                format!("array:string:file://{}", path.display()).as_str(),
            ])
            .spawn()
            .map_err(|e| {
                crate::error::DejaVuError::FileOperationFailed(format!(
                    "Failed to reveal {}: {}",
                    path.display(),
                    e
                ))
            })?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    pub fn reveal(path: &Path) -> Result<()> {
        std::process::Command::new("explorer")
            .arg("/select,")
            .arg(path)
            .spawn()
            .map_err(|e| {
                crate::error::DejaVuError::FileOperationFailed(format!(
                    "Failed to reveal {}: {}",
                    path.display(),
                    e
                ))
            })?;
        Ok(())
    }
}
