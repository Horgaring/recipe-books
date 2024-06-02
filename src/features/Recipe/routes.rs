use rocket::{ post, State};
use rocket::form::{Form};
use rocket::http::CookieJar;
use rocket::serde::json::Json;
use sqlx::Pool;
use sqlx_postgres::Postgres;
use crate::domain::entity::recipe::Recipe;
use crate::error::{ CustomError};
use crate::features;
use crate::persistence::auth::jwt;

#[post("/",data="<recipe>")]
pub async fn create(
    recipe:  Form<Recipe>,
    pool: &State<Pool<Postgres>>,
    cookie: &CookieJar<'_>
) -> Result<Json<Recipe>, CustomError> {
    let customer = match jwt::authorize(&cookie) {
        Ok(id) => id, // Assign user ID if authorization is successful
        Err(_) => { // Handle any type of error
            return Err(CustomError::Redirect(String::from("/auth0/login"))) // Or handle differently based on error
        }
    };
    match features::Book::repositoy::get_by_id(recipe.Book_id, &pool).await{
        Ok(book) => {
            if book.customer_id.unwrap() != customer {
                return Err(CustomError::PermissionDenied("Customer id is not equal customer id in book entity"))
            }
        }
        Err(_) => {return Err(CustomError::BadRequest(String::from("Book with some id does not exist")));}
    };
    match features::Recipe::repositoy::create(&recipe, pool).await{
        Ok(recipe) => Ok(Json(recipe)),
        Err(_) => Err(CustomError::BadRequest(String::from("book already exist")))
    }
}
