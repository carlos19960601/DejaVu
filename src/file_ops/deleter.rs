use crate::error::Result;
use std::path::Path;

pub struct FileDeleter;

impl FileDeleter {
    /// Delete a file permanently
    /// NOTE: This is irreversible!
    pub fn delete(path: &Path) -> Result<()> {
        std::fs::remove_file(path).map_err(|e| {
            crate::error::DejaVuError::FileOperationFailed(format!(
                "Failed to delete {}: {}",
                path.display(),
                e
            ))
        })?;
        Ok(())
    }

    /// Move file to trash (platform-specific)
    #[cfg(target_os = "macos")]
    pub fn move_to_trash(path: &Path) -> Result<()> {
        // macOS: Use osascript to move to trash
        let script = format!(
            "tell application \"Finder\" to delete POSIX file \"{}\"",
            path.display()
        );

        std::process::Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .output()
            .map_err(|e| {
                crate::error::DejaVuError::FileOperationFailed(format!(
                    "Failed to move to trash {}: {}",
                    path.display(),
                    e
                ))
            })?;

        Ok(())
    }

    #[cfg(target_os = "linux")]
    pub fn move_to_trash(path: &Path) -> Result<()> {
        // Linux: Use trash-cli if available, otherwise use gio
        // Try gio first (more common)
        let result = std::process::Command::new("gio")
            .arg("trash")
            .arg(path)
            .output();

        if result.is_ok() {
            return Ok(());
        }

        // Fallback to trash-cli
        std::process::Command::new("trash-put")
            .arg(path)
            .spawn()
            .map_err(|e| {
                crate::error::DejaVuError::FileOperationFailed(format!(
                    "Failed to move to trash {}. Please install 'trash-cli' or ensure gio is available: {}",
                    path.display(),
                    e
                ))
            })?;

        Ok(())
    }

    #[cfg(target_os = "windows")]
    pub fn move_to_trash(path: &Path) -> Result<()> {
        // Windows: Use PowerShell to move to recycle bin
        let script = format!(
            "Add-Type -AssemblyName System.Windows.Forms; [Windows.Forms.SendKeys]::SendWait('{{ENTER}}'); $shell = New-Object -ComObject Shell.Application; $item = $shell.Namespace(0).ParseName('{}'); $item.InvokeVerb('delete')",
            path.display().to_string().replace('\\', "\\\\")
        );

        std::process::Command::new("powershell")
            .arg("-Command")
            .arg(&script)
            .spawn()
            .map_err(|e| {
                crate::error::DejaVuError::FileOperationFailed(format!(
                    "Failed to move to trash {}: {}",
                    path.display(),
                    e
                ))
            })?;

        Ok(())
    }

    /// Delete multiple files with confirmation
    pub fn delete_multiple(paths: &[&Path]) -> Result<Vec<String>> {
        let mut deleted = Vec::new();
        let mut failed = Vec::new();

        for path in paths {
            match Self::delete(path) {
                Ok(_) => deleted.push(path.display().to_string()),
                Err(e) => failed.push(format!("{}: {}", path.display(), e)),
            }
        }

        if !failed.is_empty() {
            return Err(crate::error::DejaVuError::FileOperationFailed(
                failed.join("\n"),
            ));
        }

        Ok(deleted)
    }
}
