use sqlx::{FromRow};
use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize,FromRow, Deserialize)]
pub struct Customer {
    #[serde(rename = "sub")]
    pub id: String,
    pub name: String,
    pub email: String,
    #[serde(rename = "picture")]
    pub image_url: String,
}



