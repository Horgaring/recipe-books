use rocket::FromForm;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::uuid::Uuid;
use sqlx::types::time::PrimitiveDateTime;

#[derive(FromRow,FromForm,Serialize,Deserialize)]
pub struct Book {
    pub id: Option<Uuid>,
    pub name: String,
    pub customer_id: Option<String>,
    pub visibilitymode_id:Uuid,
    pub created_at: Option<PrimitiveDateTime>,
    pub updated_at: Option<PrimitiveDateTime>
}


