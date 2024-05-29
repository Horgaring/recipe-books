use sqlx::{Error, Pool};
use sqlx::types::Json;
use sqlx_postgres::Postgres;
use crate::domain::entity::customer::Customer;
use crate::persistence::auth::google_auth::AuthGoogleResponse;
use crate::persistence::specification::get_entity_by_id;

pub async fn create(customer: Customer,pool: &Pool<Postgres>) -> Result<Customer,Err()> {
    let customer = sqlx::query_as("INSERT INTO Customer (id ,name, email) values ($1,'$2','$3') RETURNING *")
            .bind(customer.id)
            .bind(customer.name)
            .bind(customer.email)
            .fetch_one(pool)
            .await;
    customer
}



pub(crate) async fn get_by_id(id: &uuid::Uuid, pool: &Pool<Postgres>) -> Result<Customer, Error> {
    let res = sqlx::query_as::<Postgres,Customer>("SELECT * FROM Customer WHERE id = $1")
        .bind(id.to_string())
        .fetch_one(pool)
        .await;
    res
}

