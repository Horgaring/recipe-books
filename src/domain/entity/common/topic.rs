use rocket::FromForm;
use serde::{Serialize, Deserialize};
use sqlx::types::Uuid;
use sqlx::{FromRow};

#[derive(Debug, Clone, FromRow,FromForm, Serialize, Deserialize)]
pub struct Topic {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub Book_id:Uuid,
}
