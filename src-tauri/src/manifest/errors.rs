use thiserror::Error;

use aws_sdk_s3::operation::get_object::GetObjectError;
use aws_smithy_runtime_api::client::result::SdkError;

use crate::config::ConfigError;

#[derive(Error, Debug)]
pub enum ManifestError {
    #[error("Invalid header magic")]
    InvalidMagic,

    #[error("Size mismatch after decompression")]
    SizeMismatch,

    #[error("Hash mismatch after decompression")]
    HashMismatch,

    #[error("There was no manifest in the cache")]
    NoManifestFound,

    #[error("There was only one manifest in the cache")]
    NoSecondLatestManifestFound,

    #[error("{0}")]
    AuthenticationFailed(String),

    #[error("{0}")]
    DownloadFailed(String),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("This wasn't supposed to happen! Pleaase contact support!")]
    UnexpectedError,
}

impl serde::Serialize for ManifestError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Error, Debug)]
pub enum ChunkLoadError {
    #[error("Invalid chunk magic")]
    InvalidMagic,

    #[error("Unknown version {0}")]
    UnknownVersion(u32),

    #[error("Storage type not supported (chunk is probably encrypted)")]
    UnsupportedStorage,

    #[error("Missing hash info")]
    MissingHashInfo,

    #[error("Serialization error")]
    SerializationError,

    #[error("File size mismatch")]
    IncorrectFileSize,

    #[error("Decompression failed")]
    DecompressFailure,

    #[error("Hash check failed")]
    HashCheckFailed,

    #[error("Downloading {0} failed: {1}")]
    DownloadFailed(String, String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("AWS error: {0}")]
    AWSError(#[from] SdkError<GetObjectError, aws_smithy_runtime_api::http::Response>),
}

impl serde::Serialize for ChunkLoadError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
