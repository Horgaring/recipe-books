use serde::{Serialize,Deserialize};
use sqlx::types::chrono;
use sqlx::types::Uuid;
use sqlx::{FromRow, Row};
use sqlx_postgres::PgRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Topic {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>, 
}
