use sqlx::{Column, Error, Pool, Row};
use sqlx_postgres::Postgres;
use sqlx::types::time::{PrimitiveDateTime,OffsetDateTime};
use sqlx::types::uuid::Uuid;
use crate::domain::entity::book::Book;

pub async fn create(book: &Book, pool: &Pool<Postgres>) -> Result<Book, Error> {
    let now_odt = OffsetDateTime::now_utc();
    let res = sqlx::query_as::<Postgres, Book>(
        "INSERT INTO Book (name, customer_id, created_at, updated_at,visibilitymode_id,id) VALUES ($1, $2, $3, $4,$5,$6) RETURNING *"
    )
    .bind(&book.name)
    .bind(&book.customer_id)
    .bind(PrimitiveDateTime::new(now_odt.date(), now_odt.time()))
    .bind(PrimitiveDateTime::new(now_odt.date(), now_odt.time()))
    .bind(book.visibilitymode_id)
    .bind(Uuid::new_v4())
    .fetch_one(pool)
    .await?;
    
    Ok(res)
}
pub(crate) async fn get_by_id(id: uuid::Uuid, pool: &Pool<Postgres>) -> Result<Book, Error> {
    let res = sqlx::query_as::<Postgres,Book>("SELECT * FROM Book WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(res)
}

