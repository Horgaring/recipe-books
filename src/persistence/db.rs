use sqlx::{ Pool};
use sqlx_postgres::{PgPoolOptions};
use std::env;
use sqlx::database::Database;


pub async fn get_db_pool() -> Pool<impl Database> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await.unwrap();
    pool
}

