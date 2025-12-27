use crate::models::file_info::FileInfo;

#[derive(Debug, Clone)]
pub struct DuplicateGroup {
    pub group_id: usize,
    pub files: Vec<FileInfo>,
    pub exact_hash: Option<Vec<u8>>,
    pub perceptual_hash: Option<u64>,
    pub recommended_original: usize, // index in files
}

impl DuplicateGroup {
    pub fn new(group_id: usize, files: Vec<FileInfo>) -> Self {
        let recommended_original = Self::select_original(&files);

        Self {
            group_id,
            files,
            exact_hash: None,
            perceptual_hash: None,
            recommended_original,
        }
    }

    pub fn with_exact_hash(mut self, hash: Vec<u8>) -> Self {
        self.exact_hash = Some(hash);
        self
    }

    pub fn with_perceptual_hash(mut self, hash: u64) -> Self {
        self.perceptual_hash = Some(hash);
        self
    }

    /// Select the recommended original file based on heuristics
    fn select_original(files: &[FileInfo]) -> usize {
        // Heuristics: prefer the file with the earliest modification time
        // If times are equal, prefer the shortest path (likely the original location)
        files
            .iter()
            .enumerate()
            .min_by_key(|(_, f)| {
                (
                    f.modified,
                    f.path.as_os_str().len(),
                    f.path.components().count(),
                )
            })
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    pub fn total_size(&self) -> u64 {
        self.files.iter().map(|f| f.size).sum()
    }

    pub fn wasted_space(&self) -> u64 {
        if self.files.is_empty() {
            return 0;
        }
        // All but the original are wasted
        self.total_size() - self.files[self.recommended_original].size
    }

    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    pub fn is_exact_duplicate(&self) -> bool {
        self.exact_hash.is_some()
    }
}
