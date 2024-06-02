use rocket::FromForm;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow,Serialize,FromForm)]
pub struct Ingredient {
    pub id:uuid::Uuid,
    pub(crate) name: String,
    pub recipe_id: uuid::Uuid,
    pub Book_id:Uuid,
}