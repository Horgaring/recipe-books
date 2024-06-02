
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use rocket::http::{Cookie, CookieJar};
use serde::{Deserialize, Serialize};
use time::{ OffsetDateTime};
use crate::domain::entity::customer;
use crate::domain::entity::customer::Customer;
use crate::error::CustomError;

#[derive(Serialize,Deserialize)]
pub struct Claims {
    sub: String,
    exp: i64,
    iat: i64,
    iss:String,
    name:String,
    email:String,
    picture:String
}
#[derive(Deserialize)]
pub struct JwkInfo {
    kty: String,
    kid: String,
    pub(crate) n: String,
    pub(crate) e: String,
    alg: String,
}
impl Jwks {
    pub fn find_jwk(&self, kid: &str) -> Option<&JwkInfo> {
        self.keys.iter().find(|key| key.kid == kid)
    }
}
#[derive(Deserialize)]
pub struct Jwks {
    keys: Vec<JwkInfo>,
}
const JWT_SECRET: &str = "apgnaengaughaegnh";
const ISSUER: &str = "2";
pub fn create_jwt(customer:customer::Customer) -> Result<String,jsonwebtoken::errors::Error> {
    let expiration: i64 =  OffsetDateTime::now_utc().unix_timestamp() * 1000 ;

    let claims = Claims {
        sub: customer.id,
        exp: expiration + 30 * 3600,
        iat: expiration,
        iss: ISSUER.to_string(),
        name:String::from(&customer.name),
        email : String::from(&customer.email),
        picture:String::from(customer.image_url)
    };
    let header = Header::new(Algorithm::HS256);
    Ok(encode(&header, &claims, &EncodingKey::from_secret(&JWT_SECRET.as_bytes()))?)
}

pub fn authorize(cookies: &CookieJar<'_>) -> Result<String,CustomError>{
    let token:&Cookie = match  cookies.get("token"){
        None => {return Err(CustomError::Unauthorized);}
        Some(token) => {token}
    };

    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_issuer(&[ISSUER]);
    validation.set_required_spec_claims(&["sub"]);

    let customer = match decode::<Customer>(
        &token.value(),
        &DecodingKey::from_secret(&JWT_SECRET.as_bytes()),
        &validation
    ){
        Ok(user) => {user}
        Err(_) => {return Err(CustomError::Unauthorized);}
    };
    Ok(customer.claims.id)
}