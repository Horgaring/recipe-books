use rocket::FromForm;
use serde::Serialize;
use sqlx::FromRow;
use sqlx::types::Uuid;

#[derive(FromRow,Serialize,FromForm)]
pub struct Ingredient {
    pub id:Option<Uuid>,
    pub(crate) name: String,
    pub recipe_id: Uuid,
    pub book_id:Uuid,
}