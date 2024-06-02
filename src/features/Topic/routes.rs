use rocket::{ post, State};
use rocket::form::{Form};
use rocket::http::CookieJar;
use rocket::serde::json::Json;
use sqlx::Pool;
use sqlx_postgres::Postgres;
use crate::domain::entity::common::topic;
use crate::domain::entity::common::topic::Topic;
use crate::error::CustomError;
use crate::features;
use crate::features::Topic::repository::{create};
use crate::persistence::auth::jwt;


#[post("/",data="<topic>")]
pub async fn create_endpoint(
    topic: Form<Topic>,
    pool: &State<Pool<Postgres>>,
    cookie: &CookieJar<'_>
) -> Result<Json<topic::Topic>, CustomError> {
    let customer = match jwt::authorize(&cookie) {
        Ok(id) => id, // Assign user ID if authorization is successful
        Err(_) => { // Handle any type of error
            return Err(CustomError::Redirect(String::from("/auth0/login"))); // Or handle differently based on error
        }
    };
    match features::Book::repositoy::get_by_id(topic.Book_id, &pool).await{
        Ok(book) => {
            if book.customer_id.unwrap() != customer {
                return Err(CustomError::PermissionDenied("Customer id is not equal customer id in book entity"));
            }
        }
        Err(_) => {return Err(CustomError::BadRequest(String::from("Book with some id does not exist")));}
    };
    match create(topic.into_inner(), pool.inner()).await {
        Ok(topic) => Ok(Json(topic)),
        Err(_) => Err(CustomError::BadRequest(String::from("Topic Already exist"))),

    }
}