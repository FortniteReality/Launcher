use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    #[serde(rename = "AccessToken")]
    pub access_token: String,
    #[serde(rename = "RefreshToken")]
    pub refresh_token: String,
    #[serde(rename = "AccountId")]
    pub account_id: String,
    #[serde(rename = "DisplayName")]
    pub display_name: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    #[serde(rename = "session_id")]
    pub _session_id: Option<String>,
    #[serde(rename = "token_type")]
    pub _token_type: Option<String>,
    #[serde(rename = "client_id")]
    pub _client_id: String,
    #[serde(rename = "client_service")]
    pub _client_service: String,
    pub account_id: String,
    #[serde(rename = "expires_in")]
    pub _expires_in: i32,
    #[serde(rename = "expires_at")]
    pub _expires_at: String,
    pub refresh_token: String,
    #[serde(rename = "refresh_expires")]
    pub _refresh_expires: i32,
    #[serde(rename = "refresh_expires_at")]
    pub _refresh_expires_at: String,
    #[serde(rename = "auth_method")]
    pub _auth_method: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "app")]
    pub _app: String,
    #[serde(rename = "in_app_id")]
    pub _in_app_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ClientResponse {
    pub access_token: String,
    #[serde(rename = "token_type")]
    pub _token_type: String,
    #[serde(rename = "client_id")]
    pub _client_id: String,
    #[serde(rename = "client_service")]
    pub _client_service: String,
    #[serde(rename = "expires_in")]
    pub _expires_in: i32,
    #[serde(rename = "expires_at")]
    pub _expires_at: String,
    #[serde(rename = "auth_method")]
    pub _auth_method: String,
    #[serde(rename = "app")]
    pub _app: String,
}

#[derive(Debug, Deserialize)]
pub struct ExchangeResponse {
    #[serde(rename = "expiresInSeconds")]
    pub _expires_in_seconds: i32,
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "creatingClientId")]
    pub _creating_client_id: String
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "errorCode")]
    pub _error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
    #[serde(rename = "messageVars")]
    pub _message_vars: Option<Vec<String>>,
    #[serde(rename = "numericErrorCode")]
    pub _numeric_error_code: Option<i32>,
    #[serde(rename = "originatingService")]
    pub _originating_service: Option<String>,
    #[serde(rename = "intent")]
    pub _intent: Option<String>,
    #[serde(rename = "error_description")]
    pub _error_description: Option<String>,
    #[serde(rename = "error")]
    pub _error: Option<String>,
}