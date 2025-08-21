use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AssetItemResponse {
    #[serde(rename = "signature")]
    pub _signature: Option<String>,
    #[serde(rename = "distribution")]
    pub distribution: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "additionalDistributions")]
    pub additional_distributions: Option<Vec<String>>
}

#[derive(Debug, Deserialize)]
pub struct AssetsResponse {
    #[serde(rename = "appName")]
    pub app_name: String,
    #[serde(rename = "labelName")]
    pub label_name: String,
    #[serde(rename = "buildVersion")]
    pub build_version: String,
    #[serde(rename = "catalogItemId")]
    pub catalog_item_id: String,
    #[serde(rename = "metadata")]
    pub _metadata: HashMap<String, String>,
    #[serde(rename = "expires")]
    pub _expires: String,
    #[serde(rename = "items")]
    pub items: HashMap<String, AssetItemResponse>,
    #[serde(rename = "assetId")]
    pub asset_id: String,
}