use sqlx::{ Pool};
use sqlx_postgres::Postgres;
use sqlx::types::uuid::Uuid;
use crate::domain::entity::common::topic;
use crate::domain::entity::common::topic::Topic;
use crate::error::CustomError;

pub async fn get_by_name(name: &str,book_id:uuid::Uuid, pool: &Pool<Postgres>) -> Result<Vec<topic::Topic>, sqlx::Error> {
    let topic: Vec<topic::Topic> = sqlx::query_as("SELECT * FROM Topic WHERE name = $1 AND Book_id = $2 RETURNING *")
        .bind(name)
        .bind(book_id)
        .fetch_all(pool)
        .await?;

    Ok(topic)
}
pub async fn get_by_id(id: uuid::Uuid, pool: &Pool<Postgres>) -> Result<topic::Topic, sqlx::Error> {
    let topic = sqlx::query_as("SELECT * FROM Topic WHERE id = $1 RETURNING *")
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(topic)
}
pub async fn create(topic: topic::Topic, pool: &Pool<Postgres>) -> Result<topic::Topic, CustomError> {
    let topic: Result<Topic,sqlx::Error>  = sqlx::query_as("INSERT INTO Topic (id,name,description, Book_id) VALUES ($1, $2, $3, $4) RETURNING *")
        .bind(Uuid::new_v4())
        .bind(&topic.name)
        .bind(&topic.description)
        .bind(&topic.Book_id)
        .fetch_one(pool)
        .await;
    match topic {
        Ok(topic) => Ok(topic),
        Err(err) => Err(CustomError::BadRequest(err.to_string()))
    }
}