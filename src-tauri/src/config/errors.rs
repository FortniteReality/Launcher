use crate::auth::AuthError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing config file")]
    MissingConfigFile,

    #[error("Missing config section")]
    MissingConfigSection,

    #[error("{0}")]
    AuthError(#[from] AuthError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("This wasn't supposed to happen! Pleaase contact support!")]
    UnexpectedError,
}

impl serde::Serialize for ConfigError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
