use reqwest::Client;

use crate::cache::get_account_info;
use crate::config::installed::{add_or_update_object, remove_object_by_artifact_id, InstalledObject};
use crate::manifest::downloader::responses::AssetsResponse;
use crate::manifest::ManifestError;

pub mod download_utils;
pub mod downloader;
pub mod errors;
pub mod progress_update;
pub mod verifier;
pub mod responses;

use crate::auth::{AccountInfo, ErrorResponse, Services};

use base64::Engine;
use base64::prelude::BASE64_STANDARD;

use std::fs;
use std::io;
use sha1::Digest;
use std::path::PathBuf;

/// Utility function that returns the path to the manifest cache folder
pub fn get_manifest_cache_path() -> io::Result<PathBuf> {
    let local_app_data = std::env::var("LOCALAPPDATA").map_err(|_| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "LOCALAPPDATA environment variable not found",
        )
    })?;

    Ok(PathBuf::from(local_app_data)
        .join("RealityLauncher")
        .join("Saved")
        .join("Manifests"))
}

/// Function that returns the latest manifest file (using the date modified) from the cache path as a b64 encoded string
pub async fn get_latest_manifest_b64() -> Result<String, ManifestError> {
    let cache_path = get_manifest_cache_path()?;
    let mut manifests: Vec<(PathBuf, std::time::SystemTime)> = Vec::new();

    for entry in fs::read_dir(&cache_path).map_err(|e| {
        ManifestError::IoError(io::Error::new(io::ErrorKind::Other, e))
    })? {
        let entry = entry.map_err(|e| {
            ManifestError::IoError(io::Error::new(io::ErrorKind::Other, e))
        })?;

        if entry.path().extension().and_then(|s| s.to_str()) == Some("manifest") {
            if let Ok(modified) = entry.metadata().and_then(|m| m.modified()) {
                manifests.push((entry.path(), modified));
            }
        }
    }

    // Sort newest first
    manifests.sort_by(|a, b| b.1.cmp(&a.1));

    if let Some((path, _)) = manifests.first() {
        let data = fs::read(path).map_err(|e| {
            ManifestError::IoError(io::Error::new(io::ErrorKind::Other, e))
        })?;
        Ok(BASE64_STANDARD.encode(data))
    } else {
        Err(ManifestError::NoManifestFound)
    }
}

/// Function that returns the second latest manifest file (using the date modified) from the cache path as a b64 encoded string
pub async fn get_second_latest_manifest_b64() -> Result<String, ManifestError> {
    let cache_path = get_manifest_cache_path()?;
    let mut manifests: Vec<(PathBuf, std::time::SystemTime)> = Vec::new();

    for entry in fs::read_dir(&cache_path).map_err(|e| {
        ManifestError::IoError(io::Error::new(io::ErrorKind::Other, e))
    })? {
        let entry = entry.map_err(|e| {
            ManifestError::IoError(io::Error::new(io::ErrorKind::Other, e))
        })?;

        if entry.path().extension().and_then(|s| s.to_str()) == Some("manifest") {
            if let Ok(modified) = entry.metadata().and_then(|m| m.modified()) {
                manifests.push((entry.path(), modified));
            }
        }
    }

    // Sort newest first
    manifests.sort_by(|a, b| b.1.cmp(&a.1));

    if manifests.len() >= 2 {
        let (path, _) = &manifests[1];
        let data = fs::read(path).map_err(|e| {
            ManifestError::IoError(io::Error::new(io::ErrorKind::Other, e))
        })?;
        Ok(BASE64_STANDARD.encode(data))
    } else {
        Err(ManifestError::NoSecondLatestManifestFound)
    }
}

/// Marks the game as deleted by removing the manifest folder and the installed object from the launcher installed data.
pub async fn mark_game_as_deleted() -> Result<(), ManifestError> {
    let cache_path = get_manifest_cache_path()?;
    if cache_path.exists() {
        fs::remove_dir_all(&cache_path).map_err(|e| {
            ManifestError::IoError(io::Error::new(io::ErrorKind::Other, e))
        })?;
    }

    match remove_object_by_artifact_id(Services::CATALOG_ID).await {
        Ok(_) => Ok(()),
        Err(_) => Ok(()) // Ignore error, as it might not exist
    }
}


/// Downloads the manifest for the latest version of the game and returns it as a byte vector.
pub async fn download_manifest() -> Result<Vec<u8>, ManifestError> {
    let account_info: &AccountInfo = get_account_info().await?;

    let client = Client::new();
    let url = format!("{0}/launcher/api/public/assets/Windows/{1}/Fortnite?label={2}", Services::LAUNCHER, Services::CATALOG_ID, Services::CATALOG_LABEL);

    let auth_response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", account_info.access_token))
        .send()
        .await?;

    if !auth_response.status().is_success() {
        if let Ok(error_response) = auth_response.json::<ErrorResponse>().await {
            let error_message = error_response
                .error_message
                .unwrap_or_else(|| "Getting current build data failed".to_string());
            return Err(ManifestError::AuthenticationFailed(error_message));
        }
        return Err(ManifestError::UnexpectedError);
    }

    let assets_response: AssetsResponse = auth_response.json().await?;

    let manifest_url = format!(
        "{}/{}",
        assets_response.items["MANIFEST"].distribution,
        assets_response.items["MANIFEST"].path
    );

    let manifest_response = client
        .get(&manifest_url)
        .header("Authorization", format!("Bearer {}", account_info.access_token))
        .send()
        .await?;

    if !manifest_response.status().is_success() {
        if let Ok(error_response) = manifest_response.json::<ErrorResponse>().await {
            let error_message = error_response
                .error_message
                .unwrap_or_else(|| "Downloading manifest file failed".to_string());
            return Err(ManifestError::AuthenticationFailed(error_message));
        }
        return Err(ManifestError::UnexpectedError);
    }

    if !manifest_response.status().is_success() {
        return Err(ManifestError::DownloadFailed(
            format!("Failed to download manifest file: {}", manifest_response.status()),
        ));
    }

    let data = manifest_response.bytes().await?;

    let mut hasher = sha1::Sha1::new();
    hasher.update(&data);
    let hash = hasher.finalize();

    let computed_hex = hex::encode(hash);

    if computed_hex != assets_response.items["MANIFEST"].hash {
        println!(
            "Hash mismatch: expected {}, got {}",
            assets_response.items["MANIFEST"].hash,
            computed_hex
        );
        return Err(ManifestError::HashMismatch);
    }

    let cache_path = get_manifest_cache_path()?;
    std::fs::create_dir_all(&cache_path).map_err(|e| {
        ManifestError::IoError(io::Error::new(io::ErrorKind::Other, e))
    })?;

    let manifest_file_path = cache_path.join(&assets_response.items["MANIFEST"].path);
    std::fs::write(&manifest_file_path, &data).map_err(|e| {
        ManifestError::IoError(io::Error::new(io::ErrorKind::Other, e))
    })?;

    println!("Manifest downloaded and saved to {:?}", manifest_file_path);

    Ok(data.to_vec())
}

/// Completes the manifest download by fetching the current build data and updating the installed object.
/// This function is called after the manifest has been downloaded and verified.
pub async fn complete_manifest_download(installation_location: &String) -> Result<(), ManifestError> {
    let account_info: &AccountInfo = get_account_info().await?;

    let client = Client::new();
    let url = format!("{0}/launcher/api/public/assets/Windows/{1}/Fortnite?label={2}", Services::LAUNCHER, Services::CATALOG_ID, Services::CATALOG_LABEL);

    let auth_response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", account_info.access_token))
        .send()
        .await?;

    if !auth_response.status().is_success() {
        if let Ok(error_response) = auth_response.json::<ErrorResponse>().await {
            let error_message = error_response
                .error_message
                .unwrap_or_else(|| "Getting current build data failed".to_string());
            return Err(ManifestError::AuthenticationFailed(error_message));
        }
        return Err(ManifestError::UnexpectedError);
    }

    let assets_response: AssetsResponse = auth_response.json().await?;

    let current_installed_object : InstalledObject = InstalledObject {
        installation_location: installation_location.clone(),
        namespace_id: assets_response.app_name.clone(),
        item_id: assets_response.asset_id.clone(),
        artifact_id: assets_response.catalog_item_id.clone(),
        app_version: assets_response.build_version.clone(),
        app_name: assets_response.app_name.clone(),
    };

    add_or_update_object(current_installed_object).await?;

    Ok(())
}

/// Gets the build version of the current manifest
pub async fn get_build_version() -> Result<String, ManifestError> {
    let account_info: &AccountInfo = get_account_info().await?;

    let client = Client::new();
    let url = format!("{0}/launcher/api/public/assets/Windows/{1}/Fortnite?label={2}", Services::LAUNCHER, Services::CATALOG_ID, Services::CATALOG_LABEL);

    let auth_response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", account_info.access_token))
        .send()
        .await?;

    if !auth_response.status().is_success() {
        if let Ok(error_response) = auth_response.json::<ErrorResponse>().await {
            let error_message = error_response
                .error_message
                .unwrap_or_else(|| "Getting current build data failed".to_string());
            return Err(ManifestError::AuthenticationFailed(error_message));
        }
        return Err(ManifestError::UnexpectedError);
    }

    let assets_response: AssetsResponse = auth_response.json().await?;
    Ok(assets_response.build_version.clone())
}