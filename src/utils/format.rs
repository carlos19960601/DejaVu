/// Format file size in human-readable format
///
/// Converts a byte count to a human-readable string representation
/// using the largest appropriate unit (GB, MB, KB, or B).
///
/// # Arguments
/// * `bytes` - The size in bytes
///
/// # Returns
/// A formatted string with the size and unit (e.g., "1.5 MB")
///
/// # Examples
/// ```
/// assert_eq!(format_size(1536), "1.5 KB");
/// assert_eq!(format_size(1048576), "1.0 MB");
/// ```
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
