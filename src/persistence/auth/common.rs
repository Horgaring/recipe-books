use rocket::serde::Deserialize;

#[derive(Deserialize)]
pub struct OidcConfig {
    pub token_endpoint: String,
    pub issuer: String,
    pub jwks_uri: String,
    pub userinfo_endpoint: String,
    pub authorization_endpoint: String,
    pub revocation_endpoint: String,
}