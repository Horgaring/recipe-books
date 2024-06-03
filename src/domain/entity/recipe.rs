use rocket::FromForm;
use serde::Serialize;
use sqlx::FromRow;
use sqlx::types::uuid::Uuid;
use sqlx::types::time::PrimitiveDateTime;

#[derive(FromRow,FromForm,Serialize)]
pub struct Recipe {
    id: Option<Uuid>,
    pub name: String,
    pub instruction: String,
    pub topic_id: uuid::Uuid,
    created_at: Option<PrimitiveDateTime>,
    updated_at: Option<PrimitiveDateTime>,
    pub book_id:Uuid,
}