#[derive(Debug, Clone)]
pub struct Services;

impl Services {
    pub const ACCOUNT: &'static str = "https://account-public-service-prod.realityfn.org";
    pub const FORTNITE: &'static str = "https://fortnite-public-service.realityfn.org";
    pub const FRIENDS: &'static str = "https://friends-public-service-prod.realityfn.org";
    pub const LAUNCHER: &'static str = "https://launcher-public-service-prod.realityfn.org";
    pub const CATALOG: &'static str = "https://catalog-public-service-prod06.realityfn.org";

    pub const CATALOG_ID: &'static str = "REALITY_ALPHA_TEST_ID";
    pub const CATALOG_LABEL: &'static str = "Live";
}

#[derive(Debug, Clone)]
pub struct ClientCredentials;

impl ClientCredentials {
    pub const CLIENT_ID: &'static str = "06591d2050b74c969d1f2bc3046aa9de";
    pub const CLIENT_SECRET: &'static str = "2f9f851728c944d28bd4d6195260d8f6";
}
