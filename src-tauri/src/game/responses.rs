
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameInfo {
    pub id: String,
    pub title: String,
    pub image: String,
    pub badge: String,
    pub installed: bool,
    pub description: String,
    pub version: String,
    pub screenshots: Vec<String>
}

pub type CatalogResponse = HashMap<String, CatalogItem>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogItem {
    pub id: String,
    pub title: String,
    pub description: String,
    pub key_images: Vec<KeyImage>,
    pub categories: Vec<Category>,
    pub namespace: String,
    pub status: String,
    pub creation_date: String,
    pub last_modified_date: String,
    pub custom_attributes: Option<HashMap<String, CustomAttribute>>,
    pub entitlement_name: String,
    pub entitlement_type: String,
    pub item_type: String,
    pub release_info: Vec<ReleaseInfo>,
    pub developer: String,
    pub developer_id: String,
    pub eula_ids: Vec<String>,
    pub end_of_support: bool,
    pub age_gatings: HashMap<String, serde_json::Value>,
    pub application_id: String,
    pub requires_secure_account: Option<bool>,
    pub unsearchable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyImage {
    #[serde(rename = "type")]
    pub image_type: String,
    pub url: String,
    pub md5: String,
    pub width: u32,
    pub height: u32,
    pub size: u32,
    pub uploaded_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomAttribute {
    #[serde(rename = "type")]
    pub attribute_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseInfo {
    pub id: String,
    pub app_id: String,
    pub compatible_apps: Vec<String>,
    pub platform: Vec<String>,
}