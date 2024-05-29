use crate::domain::entity::common::entity::Entity;
use sqlx::{FromRow, Row};
use sqlx_postgres::PgRow;
use uuid::Uuid;
use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize,FromRow, Deserialize)]
pub struct Customer {
    #[serde(rename = "sub")]
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(rename = "picture")]
    pub image_url: String,
}



