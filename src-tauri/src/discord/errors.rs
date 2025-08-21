use serde::ser::StdError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DiscordError {
    #[error("Discord Client Not Connected")]
    NotConnected,

    #[error("Discord Client Not Initialized")]
    NotInitialized,

    #[error("Dynamic Error: {0}")]
    DynError(#[from] Box<dyn StdError>),

    #[error("This wasn't supposed to happen! Pleaase contact support!")]
    UnexpectedError,
}

impl serde::Serialize for DiscordError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
