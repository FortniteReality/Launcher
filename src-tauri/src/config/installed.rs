use serde::{Deserialize, Serialize};
use std::io;
use std::path::PathBuf;
use tokio::fs;

use crate::config::ConfigError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstalledObject {
    #[serde(rename = "InstallLocation")] pub installation_location: String,
    #[serde(rename = "NamespaceId")] pub namespace_id: String,
    #[serde(rename = "ItemId")] pub item_id: String,
    #[serde(rename = "ArtifactId")] pub artifact_id: String,
    #[serde(rename = "AppVersion")] pub app_version: String,
    #[serde(rename = "AppName")] pub app_name: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LauncherInstalled {
    #[serde(rename = "InstallationList")] pub installation_list: Vec<InstalledObject>,
}

fn get_launcher_installed_path() -> io::Result<PathBuf> {
    let local_app_data = std::env::var("LOCALAPPDATA").map_err(|_| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "LOCALAPPDATA environment variable not found",
        )
    })?;

    Ok(PathBuf::from(local_app_data)
        .join("RealityLauncher")
        .join("LauncherInstalled.dat"))
}

async fn read_launcher_installed_data() -> Result<LauncherInstalled, ConfigError> {
    let path = get_launcher_installed_path()?;
    match fs::read_to_string(path).await {
        Ok(content) => {
            let data = serde_json::from_str(&content)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            Ok(data)
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(LauncherInstalled::default()),
        Err(e) => Err(ConfigError::IoError(e)),
    }
}

async fn write_launcher_installed_data(data: &LauncherInstalled) -> Result<(), ConfigError> {
    let path = get_launcher_installed_path()?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }
    let content = serde_json::to_string_pretty(data)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    fs::write(path, content)
        .await
        .map_err(|e| ConfigError::IoError(e))
}

pub async fn get_object_by_artifact_id(artifact_id: &str) -> Result<InstalledObject, ConfigError> {
    let data = read_launcher_installed_data().await?;
    data.installation_list
        .into_iter()
        .find(|obj| obj.artifact_id == artifact_id)
        .ok_or_else(|| {
            ConfigError::IoError(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Artifact ID '{}' not found", artifact_id),
            ))
        })
}

pub async fn update_object_by_artifact_id(
    updated_object: InstalledObject,
) -> Result<(), ConfigError> {
    let mut data = read_launcher_installed_data().await?;
    let artifact_id = updated_object.artifact_id.clone();

    if let Some(object_ref) = data
        .installation_list
        .iter_mut()
        .find(|obj| obj.artifact_id == artifact_id)
    {
        *object_ref = updated_object;
    } else {
        return Err(ConfigError::IoError(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Cannot update: Artifact ID '{}' not found", artifact_id),
        )));
    }

    write_launcher_installed_data(&data).await
}

pub async fn remove_object_by_artifact_id(artifact_id: &str) -> Result<(), ConfigError> {
    let mut data = read_launcher_installed_data().await?;
    
    let initial_len = data.installation_list.len();
    data.installation_list.retain(|obj| obj.artifact_id != artifact_id);
    
    if data.installation_list.len() == initial_len {
        return Err(ConfigError::IoError(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Cannot remove: Artifact ID '{}' not found", artifact_id),
        )));
    }
    
    write_launcher_installed_data(&data).await
}

pub async fn add_or_update_object(new_object: InstalledObject) -> Result<(), ConfigError> {
    let mut data = read_launcher_installed_data().await?;
    let artifact_id = new_object.artifact_id.clone();

    if let Some(position) = data
        .installation_list
        .iter()
        .position(|obj| obj.artifact_id == artifact_id)
    {
        data.installation_list[position] = new_object;
    } else {
        data.installation_list.push(new_object);
    }

    write_launcher_installed_data(&data).await
}
