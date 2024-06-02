use rocket::FromForm;
use serde::Serialize;
use sqlx::FromRow;
use time::PrimitiveDateTime;
use uuid::Uuid;

#[derive(FromRow,FromForm,Serialize)]
pub struct Recipe {
    id: uuid::Uuid,
    pub name: String,
    pub instruction: String,
    pub topic_id: uuid::Uuid,
    created_at: PrimitiveDateTime,
    updated_at: PrimitiveDateTime,
    pub Book_id:Uuid,
}