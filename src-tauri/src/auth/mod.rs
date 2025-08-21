use crate::cache::{set_account_info, get_account_info, set_client_token};
use crate::config::{ConfigError, get_remember_me_value, save_user_login};

pub mod errors;
pub mod services;
pub mod responses;

use base64::prelude::*;
use reqwest::Client;
use std::collections::HashMap;

pub use errors::AuthError;
pub use responses::*;
pub use services::{ClientCredentials, Services};

pub async fn login_client() -> Result<String, AuthError> {
    let client = Client::new();
    let url = format!("{}/account/api/oauth/token", Services::ACCOUNT);
    let client_info: String = BASE64_STANDARD.encode(format!(
        "{}:{}",
        ClientCredentials::CLIENT_ID,
        ClientCredentials::CLIENT_SECRET
    ));

    let mut form_data = HashMap::new();
    form_data.insert("grant_type", "client_credentials");

    let response = client
        .post(&url)
        .header("Client-Type", "application/x-www-form-urlencoded")
        .header("Authorization", format!("Basic {}", client_info))
        .form(&form_data)
        .send()
        .await?;

    let client_response: ClientResponse = response.json().await?;
    set_client_token(client_response.access_token.clone()).await?;
    Ok(client_response.access_token)
}

pub async fn login_user(email: String, password: String) -> Result<AccountInfo, AuthError> {
    if email.is_empty() || password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    let client = Client::new();
    let url = format!("{}/account/api/oauth/token", Services::ACCOUNT);
    let client_info: String = BASE64_STANDARD.encode(format!(
        "{}:{}",
        ClientCredentials::CLIENT_ID,
        ClientCredentials::CLIENT_SECRET
    ));

    let mut form_data = HashMap::new();
    form_data.insert("grant_type", "password");
    form_data.insert("username", &email);
    form_data.insert("password", &password);

    let response = client
        .post(&url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Authorization", format!("Basic {}", client_info))
        .form(&form_data)
        .send()
        .await?;

    if response.status() != 200 {
        if let Ok(error_response) = response.json::<ErrorResponse>().await {
            let error_message = error_response
                .error_message
                .unwrap_or_else(|| "Authentication failed".to_string());
            return Err(AuthError::AuthenticationFailed(error_message));
        }
        return Err(AuthError::UnexpectedError);
    }

    let login_response: LoginResponse = response.json().await?;
    let account_info: AccountInfo = AccountInfo {
        access_token: login_response.access_token,
        refresh_token: login_response.refresh_token,
        account_id: login_response.account_id,
        display_name: login_response.display_name,
    };

    set_account_info(account_info.clone()).await?;
    Ok(account_info)
}

pub async fn login_user_refresh(refresh_token: &String) -> Result<AccountInfo, AuthError> {
    if refresh_token.is_empty() {
        return Err(AuthError::MissingRefresh);
    }

    let client = Client::new();
    let url = format!("{}/account/api/oauth/token", Services::ACCOUNT);
    let client_info: String = BASE64_STANDARD.encode(format!(
        "{}:{}",
        ClientCredentials::CLIENT_ID,
        ClientCredentials::CLIENT_SECRET
    ));

    let mut form_data = HashMap::new();
    form_data.insert("grant_type", "refresh_token");
    form_data.insert("refresh_token", &refresh_token);

    let response = client
        .post(&url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Authorization", format!("Basic {}", client_info))
        .form(&form_data)
        .send()
        .await?;

    if response.status() != 200 {
        if let Ok(error_response) = response.json::<ErrorResponse>().await {
            let error_message = error_response
                .error_message
                .unwrap_or_else(|| "Authentication failed".to_string());
            return Err(AuthError::AuthenticationFailed(error_message));
        }
        return Err(AuthError::UnexpectedError);
    }

    let login_response: LoginResponse = response.json().await?;
    let account_info: AccountInfo = AccountInfo {
        access_token: login_response.access_token,
        refresh_token: login_response.refresh_token,
        account_id: login_response.account_id,
        display_name: login_response.display_name,
    };

    set_account_info(account_info.clone()).await?;

    let remember_me = get_remember_me_value().await.map_err(|_| AuthError::UnexpectedError)?;
    save_user_login(remember_me, account_info.refresh_token.clone()).await.map_err(|_| AuthError::UnexpectedError)?;

    Ok(account_info)
}

pub async fn generate_exchange() -> Result<String, ConfigError> {
    let account_info: &AccountInfo = get_account_info().await?;

    let client = Client::new();
    let url = format!("{}/account/api/oauth/exchange", Services::ACCOUNT);

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", account_info.access_token))
        .send()
        .await?;

    if response.status() != 200 {
        if let Ok(error_response) = response.json::<ErrorResponse>().await {
            let error_message = error_response
                .error_message
                .unwrap_or_else(|| "Exchange code generation failed".to_string());
            return Err(ConfigError::AuthError(AuthError::AuthenticationFailed(error_message)));
        }
        return Err(ConfigError::AuthError(AuthError::UnexpectedError));
    }

    let exchange_response: ExchangeResponse = response.json().await?;
    Ok(exchange_response.code)
}