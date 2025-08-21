use thiserror::Error;

use crate::auth::AccountInfo;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Missing email or password")]
    MissingCredentials,

    #[error("Missing refresh token")]
    MissingRefresh,

    #[error("Missing access token cache")]
    MissingAccessTokenCache,

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("{0}")]
    AuthenticationFailed(String),

    #[error("Error setting client credentials config: {0}")]
    ClientCredentialsConfigError(#[from] tokio::sync::SetError<String>),

    #[error("Error setting account info config: {0}")]
    AccountInfoConfigError(#[from] tokio::sync::SetError<AccountInfo>),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("This wasn't supposed to happen! Pleaase contact support!")]
    UnexpectedError,
}

impl serde::Serialize for AuthError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
