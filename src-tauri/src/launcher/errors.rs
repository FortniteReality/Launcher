use thiserror::Error;

use crate::{config::ConfigError, manifest::errors::ChunkLoadError};

#[derive(Error, Debug)]
pub enum LaunchError {
    #[error("Fortnite doesn't exist")]
    MissingFortnite,

    #[error("Path invalid: {0}")]
    InvalidPath(String),

    #[error("EAC doesn't exist")]
    MissingEAC,

    #[error("Fortnite Launcher doesn't exist")]
    MissingLauncher,

    #[error("Equinox doesn't exist")]
    MissingAntiCheat,

    #[error("Invalid arguments {0}")]
    InvalidArguments(String),

    #[error("Dll Load Failed {0}")]
    DllLoadError(String),

    #[error("Dll Call Failed {0}")]
    DllCallError(String),

    #[error("Dll not loaded")]
    DllNotLoaded,

    #[error("Launch Failed {0}")]
    LaunchFailed(String),

    #[error("{0} failed to start: {1}")]
    FailedToStart(String, std::io::Error),

    #[error("Error waiting on Fortite process: {0}")]
    FailedToWait(std::io::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),

    #[error("CloudFlare R2 error: {0}")]
    CloudFlareError(#[from] ChunkLoadError),

    #[error("This wasn't supposed to happen! Pleaase contact support!")]
    UnexpectedError,
}

impl serde::Serialize for LaunchError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}