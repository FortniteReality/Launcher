use crate::auth::{login_client, AccountInfo, AuthError};
use crate::config::{fetch_saved_user_login, ConfigError};
use tokio::sync::{OnceCell, SetError};

static ACCOUNT_INFO: OnceCell<AccountInfo> = OnceCell::const_new();
static CLIENT_TOKEN: OnceCell<String> = OnceCell::const_new();

pub async fn get_account_info() -> Result<&'static AccountInfo, ConfigError> {
    ACCOUNT_INFO.get_or_try_init(fetch_saved_user_login).await
}

pub async fn set_account_info(account_info: AccountInfo) -> Result<(), SetError<AccountInfo>> {
    ACCOUNT_INFO.set(account_info)
}

pub async fn get_client_token() -> Result<&'static String, AuthError> {
    CLIENT_TOKEN.get_or_try_init(login_client).await
}

pub async fn set_client_token(client_token: String) -> Result<(), SetError<String>> {
    CLIENT_TOKEN.set(client_token)
}
