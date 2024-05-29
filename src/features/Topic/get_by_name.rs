use rocket::{ get, serde::json::Json, State};
use std::io;
use sqlx::Pool;
use sqlx_postgres::Postgres;
use crate::domain::entity::common::topic;

async fn get_by_name(name: &str, pool: &Pool<Postgres>) -> Result<Vec<topic::Topic>, sqlx::Error> {
    let topic: Vec<topic::Topic> = sqlx::query_as("SELECT * FROM Topic WHERE name = $1")
        .bind(name)
        .fetch_all(pool)
        .await?;
    
    Ok(topic)
}

#[get("/customers/<id>")]
pub async fn get_by_name_endpoint(
    id: &str,
    pool: &State<Pool<Postgres>>
) -> Result<Json<Vec<topic::Topic>>, io::Error> {
    match get_by_name(id, pool.inner()).await {
        Ok(topic) => Ok(Json(topic)),
        Err(_) => Err(io::Error::new(io::ErrorKind::NotFound, "User not found")),
        
    }
}