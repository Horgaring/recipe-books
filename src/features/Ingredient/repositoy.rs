use sqlx::{Error, Pool};
use sqlx_postgres::Postgres;
use uuid::Uuid;
use crate::domain::entity::common::ingredient::Ingredient;


pub async fn create(ingredient: &Ingredient, pool: &Pool<Postgres>) -> Result<Ingredient, Error> {
    let res = sqlx::query_as::<Postgres, Ingredient>(
        "INSERT INTO Ingredient (name, id, recipe_id) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&ingredient.name)
    .bind(Uuid::new_v4())
    .bind(ingredient.recipe_id)
    .fetch_one(pool)
    .await?;
    Ok(res)
}
