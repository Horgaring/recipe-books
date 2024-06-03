use sqlx::{Error, Pool};
use sqlx_postgres::Postgres;
use time::{ OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;
use crate::domain::entity::customer::Customer;
use crate::domain::entity::recipe::Recipe;

pub async fn create(recipe: &Recipe, pool: &Pool<Postgres>) -> Result<Recipe, Error> {
    let now_odt = OffsetDateTime::now_utc();
    let res = sqlx::query_as::<Postgres, Recipe>(
        "INSERT INTO Recipe (name, topic_id, created_at, updated_at,instruction,id,book_id) VALUES ($1, $2, $3, $4,$5,$6,$7) RETURNING *"
    )
    .bind(&recipe.name)
    .bind(&recipe.topic_id)
    .bind(PrimitiveDateTime::new(now_odt.date(), now_odt.time()))
    .bind(PrimitiveDateTime::new(now_odt.date(), now_odt.time()))
    .bind(&recipe.instruction)
    .bind(Uuid::new_v4())
    .bind(recipe.book_id)
    .fetch_one(pool)
    .await?;
    Ok(res)
}
pub(crate) async fn get_by_id(id: uuid::Uuid, pool: &Pool<Postgres>) -> Result<Customer, Error> {
    let res = sqlx::query_as::<Postgres,Customer>("SELECT * FROM Recipe WHERE id = $1")
        .bind(id.to_string())
        .fetch_one(pool)
        .await;
    res
}
