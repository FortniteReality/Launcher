use thiserror::Error;

use crate::config::ConfigError;

#[derive(Error, Debug)]
pub enum GameInfoError {
    #[error("{0}")]
    AuthenticationFailed(String),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Config Error: {0}")]
    ConfigError(#[from] ConfigError),

    #[error("This wasn't supposed to happen! Pleaase contact support!")]
    UnexpectedError,
}

impl serde::Serialize for GameInfoError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
