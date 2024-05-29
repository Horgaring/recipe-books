use std::io;
use rocket::{get, State};
use rocket::serde::json::Json;
use sqlx::Pool;
use sqlx_postgres::Postgres;
use crate::domain::entity::customer::Customer;

#[get("/customers/<id>")]
pub async fn get_by_id_endpoint(
    id: rocket::serde::uuid::Uuid,
    pool: &State<Pool<Postgres>>
) -> Result<Json<Customer>, io::Error> {
    match crate::features::Customer::Repository::get_by_id(&id, pool.inner()).await {
        Some(user) => Ok(user),
        Err(err) => Err(io::Error::new(io::ErrorKind::NotFound, "User not found")),
    }
}
