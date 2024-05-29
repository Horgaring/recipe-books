use sqlx::Pool;
use sqlx_postgres::{Postgres,PgRow};

use crate::domain::entity::customer::Customer;

pub async fn get_entity_by_id<T>(id: &uuid::Uuid,table_name: &str, pool: &Pool<Postgres>) -> Result<T, sqlx::Error> 
    where T: for<'a> sqlx::FromRow<'a,
        PgRow>, T: Send,
        T : Unpin,{
    let res = sqlx::query_as::<Postgres,T>("SELECT * FROM $1 WHERE id = $2")
        .bind(table_name)
        .bind(id.to_string())
        .fetch_one(pool)
        .await?;
    Ok(res)
}
pub async fn create_user<T>(customer: Customer, pool: &Pool<Postgres>) -> Result<bool, sqlx::Error>{
    sqlx::query(format!("INSERT INTO Customer (id ,name, email) values (gen_random_uuid(),'{}','{}') ;",customer.name,customer.email).as_str())
        .execute(pool)
        .await?;

    Ok(true)
}
