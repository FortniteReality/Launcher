use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FriendResponse {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "groups")]
    pub _groups: Option<Vec<String>>,
    #[serde(rename = "alias")]
    pub _alias: Option<String>,
    #[serde(rename = "note")]
    pub _note: Option<String>,
    #[serde(rename = "favorite")]
    pub _favorite: Option<bool>,
    #[serde(rename = "created")]
    pub _created: String
}

#[derive(Debug, Deserialize)]
pub struct FriendSettingsResponse {
    #[serde(rename = "acceptInvites")]
    pub _accept_invites: String,
    #[serde(rename = "mutualPrivacy")]
    pub _mutual_privacy: String
}

#[derive(Debug, Deserialize)]
pub struct LimitsReachedResponse {
    #[serde(rename = "incoming")]
    pub _incoming: bool,
    #[serde(rename = "outgoing")]
    pub _outgoing: bool,
    #[serde(rename = "accepted")]
    pub _accepted: bool
}

#[derive(Debug, Deserialize)]
pub struct SummaryResponse {
    #[serde(rename = "friends")]
    pub friends: Vec<FriendResponse>,
    #[serde(rename = "incoming")]
    pub incoming: Vec<FriendResponse>,
    #[serde(rename = "outgoing")]
    pub outgoing: Vec<FriendResponse>,
    #[serde(rename = "suggested")]
    pub _suggested: Vec<FriendResponse>,
    #[serde(rename = "blocklist")]
    pub blocklist: Vec<FriendResponse>,
    #[serde(rename = "settings")]
    pub _settings: FriendSettingsResponse,
    #[serde(rename = "limitsReached")]
    pub _limits_reached: LimitsReachedResponse
}

#[derive(Debug, Deserialize)]
pub struct AuthIdResponse {
    #[serde(rename = "Id")]
    pub _id: String,
    #[serde(rename = "Type")]
    pub _type: String,
}


#[derive(Debug, Deserialize)]
pub struct ExternalAuthsResponse {
    #[serde(rename = "accountId")]
    _account_id: String,
    #[serde(rename = "active")]
    _active: bool,
    #[serde(rename = "authIds")]
    _auth_ids: Vec<AuthIdResponse>,
    #[serde(rename = "dateAdded")]
    _date_added: String,
    #[serde(rename = "externalAuthId")]
    _external_auth_id: String,
    #[serde(rename = "externalAuthType")]
    _external_auth_type: String,
    #[serde(rename = "externalDisplayName")]
    _external_display_name: String,
    #[serde(rename = "id")]
    _id: String,
    #[serde(rename = "isActive")]
    _is_active: bool,
    #[serde(rename = "type")]
    _type: String
}

#[derive(Debug, Deserialize)]
pub struct DisplayNameLookupResponse {
    #[serde(rename = "id")]
    pub _id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "externalAuths")]
    pub _external_auths: Option<ExternalAuthsResponse>
}