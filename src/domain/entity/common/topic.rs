use rocket::FromForm;
use serde::{Serialize, Deserialize};
use sqlx::types::Uuid;
use sqlx::{FromRow};

#[derive(Debug, Clone, FromRow,FromForm, Serialize, Deserialize)]
pub struct Topic {
    pub id: Option<Uuid>,
    pub name: String,
    pub description: String,
    pub book_id:Uuid,
}
