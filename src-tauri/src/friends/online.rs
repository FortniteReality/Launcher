use reqwest::Client;

use crate::auth::{AccountInfo, AuthError, Services};
use crate::auth::responses::ErrorResponse;

use crate::cache::get_account_info;
use crate::friends::errors::FriendError;

use crate::friends::responses::{DisplayNameLookupResponse, SummaryResponse};

/// Utility function to retrieve the friend summary
async fn get_friend_summary() -> Result<SummaryResponse, FriendError> {
    let account_info: &AccountInfo = get_account_info().await?;

    let client = Client::new();
    let url = format!("{0}/friends/api/v1/{1}/summary", Services::FRIENDS, account_info.account_id);

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", account_info.access_token))
        .send()
        .await?;

    if response.status() != 200 {
        if let Ok(error_response) = response.json::<ErrorResponse>().await {
            let error_message = error_response
                .error_message
                .unwrap_or_else(|| "Fetching friend summary failed".to_string());
            return Err(FriendError::AuthError(AuthError::AuthenticationFailed(error_message)));
        }
        return Err(FriendError::UnexpectedError);
    }

    Ok(response.json().await?)
}


/// Accepts or sends a friend request by friend account id
pub async fn accept_friend(friend_account_id: String) -> Result<(), FriendError> {
    let account_info: &AccountInfo = get_account_info().await?;

    let client = Client::new();
    let url = format!("{0}/friends/api/v1/{1}/friends/{2}", Services::FRIENDS, account_info.account_id, friend_account_id);

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", account_info.access_token))
        .send()
        .await?;

    if response.status() != 204 {
        if let Ok(error_response) = response.json::<ErrorResponse>().await {
            let error_message = error_response
                .error_message
                .unwrap_or_else(|| "Accepting friend request failed".to_string());
            return Err(FriendError::AuthError(AuthError::AuthenticationFailed(error_message)));
        }
        return Err(FriendError::UnexpectedError);
    }

    Ok(())
}

/// Declines a friend request by friend account id
pub async fn decline_friend(friend_account_id: String) -> Result<(), FriendError> {
    let account_info: &AccountInfo = get_account_info().await?;

    let client = Client::new();
    let url = format!("{0}/friends/api/v1/{1}/friends/{2}", Services::FRIENDS, account_info.account_id, friend_account_id);

    let response = client
        .delete(&url)
        .header("Authorization", format!("Bearer {}", account_info.access_token))
        .send()
        .await?;

    if response.status() != 204 {
        if let Ok(error_response) = response.json::<ErrorResponse>().await {
            let error_message = error_response
                .error_message
                .unwrap_or_else(|| "Accepting friend request failed".to_string());
            return Err(FriendError::AuthError(AuthError::AuthenticationFailed(error_message)));
        }
        return Err(FriendError::UnexpectedError);
    }

    Ok(())
}

/// Gets the account display name from an account id
pub async fn get_display_name_by_account_id(account_id: String) -> Result<String, FriendError> {
    let account_info: &AccountInfo = get_account_info().await?;

    let client = Client::new();
    let url = format!("{0}/account/api/public/account/{1}", Services::ACCOUNT, account_id);

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", account_info.access_token))
        .send()
        .await?;

    if response.status() != 200 {
        if let Ok(error_response) = response.json::<ErrorResponse>().await {
            let error_message = error_response
                .error_message
                .unwrap_or_else(|| "Fetching display name failed".to_string());
            return Err(FriendError::AuthError(AuthError::AuthenticationFailed(error_message)));
        }
        return Err(FriendError::UnexpectedError);
    }

    let lookup_response: DisplayNameLookupResponse = response.json().await?;
    Ok(lookup_response.display_name)
}

/// Gets the logged in user's current friends
pub async fn get_friends() -> Result<Vec<String>, FriendError> {
    let summary_response: SummaryResponse = get_friend_summary().await?;

    let mut account_ids: Vec<String> = vec![];
    for friend in summary_response.friends {
        account_ids.push(friend.account_id);
    }

    Ok(account_ids)
}

/// Gets the logged in user's incoming friend requests
pub async fn get_incoming_friends() -> Result<Vec<String>, FriendError> {
    let summary_response: SummaryResponse = get_friend_summary().await?;

    let mut account_ids: Vec<String> = vec![];
    for friend in summary_response.incoming {
        account_ids.push(friend.account_id);
    }

    Ok(account_ids)
}

/// Gets the logged in user's outgoing friend requests
pub async fn get_outgoing_friends() -> Result<Vec<String>, FriendError> {
    let summary_response: SummaryResponse = get_friend_summary().await?;

    let mut account_ids: Vec<String> = vec![];
    for friend in summary_response.outgoing {
        account_ids.push(friend.account_id);
    }

    Ok(account_ids)
}

/// Gets the logged in user's block list
pub async fn get_blocked_users() -> Result<Vec<String>, FriendError> {
    let summary_response: SummaryResponse = get_friend_summary().await?;

    let mut account_ids: Vec<String> = vec![];
    for friend in summary_response.blocklist {
        account_ids.push(friend.account_id);
    }

    Ok(account_ids)
}