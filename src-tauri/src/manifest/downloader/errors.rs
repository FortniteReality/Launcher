use aws_sdk_s3::primitives::ByteStreamError;
use base64::DecodeError;
use thiserror::Error;

use crate::manifest::{
    downloader::progress_update::ProgressUpdate, errors::ChunkLoadError, ManifestError,
};

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("Buffer too small")]
    BufferTooSmall,

    #[error("Download has been cancelled")]
    Cancelled,

    #[error("Download has been timed out")]
    Timeout,

    #[error("Hash mismatch after download: {0}")]
    HashMismatch(String),

    #[error("Multiple files failed downloading")]
    Multiple(Vec<DownloadError>),

    #[error("Downloading chunk {0} failed: {1}")]
    ChunkDownloadFailed(String, String),

    #[error("Repair failed: {0}")]
    RepairFailed(String),

    #[error("Io Error: {0}")]
    Io(String),

    #[error("Missing file: {0}")]
    MissingFile(String),

    #[error("Chunk corrupt: {0}")]
    ChunkCorrupt(String),

    #[error("Chunk missing")]
    ChunkMissing,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Error decoding b64: {0}")]
    DecodeError(#[from] DecodeError),

    #[error("Task joining error: {0}")]
    TaskError(#[from] tokio::task::JoinError),

    #[error("Byte stream error: {0}")]
    ByteStreamError(#[from] ByteStreamError),

    #[error("Error reading manifest: {0}")]
    ManifestError(#[from] ManifestError),

    #[error("Error loading chunk: {0}")]
    ChunkLoadError(#[from] ChunkLoadError),

    #[error("Channel closed")]
    ChannelError(#[from] tokio::sync::mpsc::error::SendError<ProgressUpdate>),

    #[error("This wasn't supposed to happen! Pleaase contact support!")]
    UnexpectedError,
}

impl serde::Serialize for DownloadError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
