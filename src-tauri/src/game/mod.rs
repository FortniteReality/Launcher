
pub mod errors;
pub mod responses;
use errors::GameInfoError;
use reqwest::Client;

use crate::auth::{AccountInfo, ErrorResponse, Services};
use crate::cache::get_account_info;
use crate::config::installed::get_object_by_artifact_id;
use crate::game::responses::{CatalogResponse, GameInfo};
use crate::manifest::downloader::get_build_version;

pub async fn fetch_current_game_data() -> Result<GameInfo, GameInfoError> {
    let account_info: &AccountInfo = get_account_info().await?;

    let client = Client::new();
    let url = format!("{0}/catalog/api/shared/bulk/items?id={1}", Services::CATALOG, Services::CATALOG_ID);

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", account_info.access_token))
        .send()
        .await?;

    if !response.status().is_success() {
        if let Ok(error_response) = response.json::<ErrorResponse>().await {
            let error_message = error_response
                .error_message
                .unwrap_or_else(|| "Fetching catalog item info failed".to_string());
            return Err(GameInfoError::AuthenticationFailed(error_message));
        }
        return Err(GameInfoError::UnexpectedError);
    }

    let catalog_response: CatalogResponse = response.json().await?;

    let build_version: String = match get_object_by_artifact_id(Services::CATALOG_ID).await {
        Ok(installed_game) => installed_game.app_version,
        Err(_) => get_build_version().await.map_err(|_| GameInfoError::UnexpectedError)?
    };

    Ok(GameInfo {
        id: catalog_response[Services::CATALOG_ID].id.clone(),
        title: catalog_response[Services::CATALOG_ID].title.clone(),
        image: catalog_response[Services::CATALOG_ID].key_images.iter()
            .find(|img| img.image_type == "Offer")
            .map_or("".to_string(), |img| img.url.clone()),
        badge: "Early Access".to_string(),
        installed: false,
        description: catalog_response[Services::CATALOG_ID].description.clone(),
        version: build_version,
        screenshots: catalog_response[Services::CATALOG_ID].key_images.iter()
            .filter(|img| img.image_type == "Screenshot")
            .map(|img| img.url.clone())
            .collect()
    })
}