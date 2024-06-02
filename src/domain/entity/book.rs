use rocket::time::PrimitiveDateTime;
use rocket::FromForm;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow,FromForm,Serialize,Deserialize)]
pub struct Book {
    pub id: Option<Uuid>,
    pub name: String,
    pub customer_id: Option<String>,
    pub visibilityMode_id:Option<Uuid>,
    pub created_at: Option<PrimitiveDateTime>,
    pub updated_at: Option<PrimitiveDateTime>
}


