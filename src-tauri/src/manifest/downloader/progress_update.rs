use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct ProgressUpdate {
    pub filename: String,
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
    pub total_files: usize
}

impl Default for ProgressUpdate {
    fn default() -> Self {
        Self {
            filename: "unknown".to_string(),
            downloaded_bytes: 0,
            total_bytes: 0,
            total_files: 0,
        }
    }
}