use std::{io, panic};
use rocket::{get, State};
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::response::{Redirect};
use serde::Deserialize;
use crate::domain::entity::customer;
use crate::features::Customer;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation, decode_header};
use sqlx::{Error, Pool};
use sqlx_postgres::Postgres;
use crate::features;
use crate::persistence::auth::common::OidcConfig;
use crate::persistence::auth::jwt::{create_jwt, Jwks};


async fn get_well_known() -> Result<OidcConfig, Box<dyn std::error::Error>> {
    let oidc_config = reqwest::get("https://accounts.google.com/.well-known/openid-configuration")
        .await?
        .json::<OidcConfig>()
        .await?;
    
    Ok(oidc_config)
} 
#[rocket::get("/callback?<code>")]
pub async fn callback(code: &str,
                      cookies: &CookieJar<'_>,
                      pool: &State<Pool<Postgres>>) -> Redirect {
    let config = get_well_known().await.unwrap();

    let client_id: String = std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let client_secret: String = std::env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set");
    let token_endpoint: String = config.token_endpoint;
    let token_request = GetTokenRequest{
        client_id,
        client_secret,
        code: code.to_string(),
        redirect_uri: "http://localhost:8000/auth0/callback".to_string(),
        grant_type: "authorization_code".to_string(),
    };
    let token = get_token(&token_endpoint, token_request).await.unwrap();

    let jwk_response = reqwest::get(&config.jwks_uri).await.unwrap().json::<Jwks>().await.unwrap();
    let jwk = jwk_response.find_jwk(decode_header(token.id_token.as_str()).unwrap().kid.unwrap().as_str()).unwrap();

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&["https://accounts.google.com"]);
    let client_id: String = std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    validation.set_audience(&[&client_id]);
    let token = decode::<customer::Customer>(token.id_token.as_str(),
        &DecodingKey::from_rsa_components(&jwk.n, &jwk.e).unwrap(),
        &validation).unwrap().claims;

    let customer: Option<customer::Customer> = match Customer::repository::create(&token, &pool).await{
        Ok(user) => Some(user),
        Err(err) => {
            if !err.to_string().contains("duplicate key") {
                panic!("{}",Error::from(io::Error::new(io::ErrorKind::Other, err.to_string())));
            }
            Some(features::Customer::repository::get_by_id(token.id, pool).await.unwrap())
        }
    };

    let cookie = Cookie::build(("token", create_jwt(customer.unwrap()).unwrap()))
        .secure(true)
        .same_site(SameSite::Strict);
    cookies.add(cookie);

    Redirect::to("/")
}

pub async fn auth(auth_endpoint: &str, auth_request: AuthGoogleRequest) -> String{
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
    Redirect::to(auth("https://accounts.google.com/o/oauth2/v2/auth", a).await)
}

async fn get_token(endpoint: &str,token_request: GetTokenRequest) -> serde_json::error::Result<AuthGoogleResponse> {
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
        .send().await.unwrap()
        .text().await.unwrap();
    serde_json::from_str(&token)
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
    pub refresh_token: Option<String>,
}
#[derive(Deserialize)]
struct Jwk{
    key: String
}
