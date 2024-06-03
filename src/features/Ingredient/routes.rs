use rocket::{ post, State};
use rocket::form::{Form};
use rocket::http::CookieJar;
use rocket::serde::json::Json;
use sqlx::Pool;
use sqlx_postgres::Postgres;
use crate::domain::entity::book::Book;
use crate::domain::entity::common::ingredient::Ingredient;
use crate::error::{ CustomError};
use crate::features;
use crate::persistence::auth::jwt;

#[post("/",data="<ingredient>")]
pub async fn create(
    ingredient:  Form<Ingredient>,
    pool: &State<Pool<Postgres>>,
    cookie: &CookieJar<'_>
) -> Result<Json<Ingredient>, CustomError> {
    let customer = match jwt::authorize(&cookie) {
        Ok(id) => id, // Assign user ID if authorization is successful
        Err(_) => { // Handle any type of error
            return Err(CustomError::Redirect(String::from("/auth0/login"))) // Or handle differently based on error
        }
    };
    match features::Book::repositoy::get_by_id(ingredient.book_id, &pool).await{
        Ok(book) => {
            if book.customer_id.unwrap() != customer {
               return Err(CustomError::PermissionDenied("Customer id is not equal customer id in book entity"))
            }
        }
        Err(_) => {return Err(CustomError::BadRequest(String::from("Book with some id does not exist")))}
    };
    match features::Ingredient::repositoy::create(&ingredient, pool).await{
        Ok(ingredient) => Ok(Json(ingredient)),
        Err(_) => Err(CustomError::BadRequest(String::from("book already exist")))
    }
}
