use rocket::{get, post, State};
use rocket::form::{Form};
use rocket::http::CookieJar;
use rocket::response::content;
use rocket::response::content::RawHtml;
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
    match features::Book::repositoy::get_by_id(recipe.book_id, &pool).await{
        Ok(book) => {
            if book.customer_id.unwrap() != customer {
                return Err(CustomError::PermissionDenied("Customer id is not equal customer id in book entity"))
            }
        }
        Err(_) => {return Err(CustomError::BadRequest(String::from("Book with some id does not exist")));}
    };
    match features::Recipe::repositoy::create(&recipe, pool).await{
        Ok(recipe) => Ok(Json(recipe)),
        Err(_) => Err(CustomError::BadRequest(String::from("recipe already exist")))
    }
}
#[get("/")]
pub async fn create_html() -> content::RawHtml<String> {
    RawHtml(r#"
          <form class="generated-form"  method="POST" action="http:\\localhost:8000/recipes"  target="_self">
<fieldset>
  <legend></legend>
  <label for="name">First name:</label><br>
  <input type="text"  name="name" value="John"><br>
  <label for="instruction">instruction:</label><br>
  <input type="text"  name="instruction" value="John"><br>
  <label for="book_id">book_id:</label><br>
  <input type="text"  name="book_id" value="John"><br>
  <label for="topic_id">topic_id:</label><br>
  <input type="text"  name="topic_id" value="John"><br>
  <input type="submit" value="Submit">
</fieldset>
</form>
    "#.to_string())
}