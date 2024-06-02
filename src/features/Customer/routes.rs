use rocket::{get, State};
use rocket::serde::json::Json;
use sqlx::Pool;
use sqlx_postgres::Postgres;
use crate::domain::entity::customer::Customer;
use crate::error::CustomError;

#[get("/<id>")]
pub async fn get_by_id_endpoint(
    id: String,
    pool: &State<Pool<Postgres>>
) -> Result<Json<Customer>, CustomError> {

    match crate::features::Customer::repository::get_by_id(id, pool.inner()).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(CustomError::BadRequest("USer not found".to_string())),
    }
}
