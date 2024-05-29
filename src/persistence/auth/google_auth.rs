use std::collections::HashMap;
use chrono::format;
use reqwest::redirect;
use rocket::get;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::response::{self, content, Redirect};
use rocket::http::uri::Uri;
use serde::Deserialize;
use crate::features::Customer;

async fn get_well_known() -> Result<OidcConfig, Box<dyn std::error::Error>> {
    let oidc_config = reqwest::get("https://accounts.google.com/.well-known/openid-configuration")
        .await?
        .json::<OidcConfig>()
        .await?;

    Ok((oidc_config))
} 
#[rocket::get("/callback?<code>")]
pub async fn callback(code: &str,cookies: &CookieJar<'_>) -> Redirect {
    let client_id: String = std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let client_secret: String = std::env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set");
    let token_endpoint: String = get_well_known().await.unwrap()
        .token_endpoint;  
    let token_request = GetTokenRequest{
        client_id,
        client_secret,
        code: code.to_string(),
        redirect_uri: "http://localhost:8000/auth0/callback".to_string(),
        grant_type: "authorization_code".to_string(),
    };
    let token = get_token(&token_endpoint, token_request).await;
    let cookie = Cookie::build(("token", serde_json::to_string(&token).unwrap()))
        .secure(true)
        .same_site(SameSite::Strict);
    cookies.add(cookie);

    Redirect::to("/")
}
pub async fn authorize(auth_endpoint: &str, auth_request: AuthGoogleRequest) -> String{
    let mut params = vec![
        ("client_id", auth_request.client_id),
        ("scope", auth_request.scope),
        ("redirect_uri", auth_request.redirect_uri.to_string()),
        ("response_type", auth_request.response_type),
        ("access_type", "offline".to_string()),
    ];
    if let Some(nonce) = auth_request.nonce {
        params.push(("nonce", nonce));
    }
    let url = reqwest::Url::parse_with_params(auth_endpoint,params).unwrap();
    url.to_string()
    // reqwest::get(url)
    //     .await; 
}
#[get("/login")]
pub async fn external() -> Redirect {
    let a: AuthGoogleRequest = AuthGoogleRequest{
        client_id: std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set"),
        redirect_uri: "http://localhost:8000/auth0/callback".to_string(),
        response_type: "code".to_string(),
        scope: "profile email openid".to_string(),
        nonce: Some("xyz".to_string()),
    };
    Redirect::to(authorize("https://accounts.google.com/o/oauth2/v2/auth",a).await)
}
async fn get_token(endpoint: &str,token_request: GetTokenRequest) -> AuthGoogleResponse {
    let client = reqwest::Client::new();
    let form_data = [
        ("client_id", token_request.client_id),
        ("client_secret", token_request.client_secret),
        ("code", token_request.code),
        ("redirect_uri", token_request.redirect_uri.to_string()),
        ("grant_type", token_request.grant_type),
    ];
    let token = client.post(endpoint)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&form_data)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    serde_json::from_str(&token).unwrap()
}
pub struct AuthGoogleRequest {
    pub client_id: String,
    pub scope: String,
    pub redirect_uri: String,
    pub response_type: String,
    pub nonce: Option<String>,
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct GetTokenRequest {
    pub client_id: String,
    pub client_secret: String,
    pub code: String,
    pub redirect_uri: String,
    pub grant_type: String,
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AuthGoogleResponse {
    pub access_token: String,
    pub id_token: String,
    pub refresh_token: String,
}
#[derive(Deserialize)]
struct OidcConfig {
    token_endpoint: String,
    issuer: String,
    jwks_uri: String,
    userinfo_endpoint: String,
    authorization_endpoint: String,
    revocation_endpoint: String,
}