use rocket::{get, post, State};
use rocket::form::{Form};
use rocket::http::CookieJar;
use rocket::response::content;
use rocket::response::content::RawHtml;
use rocket::serde::json::Json;
use sqlx::Pool;
use sqlx_postgres::Postgres;
use crate::domain::entity::book::Book;
use crate::error::{ CustomError};
use crate::features;
use crate::persistence::auth::jwt;

#[post("/",data="<book>")]
pub async fn create(
    mut book:  Form<Book>,
    pool: &State<Pool<Postgres>>,
    cookie: &CookieJar<'_>
) -> Result<Json<Book>, CustomError> {
    let customer = match jwt::authorize(&cookie) {
        Ok(id) => id, // Assign user ID if authorization is successful
        Err(_) => { // Handle any type of error
            return Err(CustomError::Redirect(String::from("/auth0/login"))); // Or handle differently based on error
        }
    };
    book.customer_id = Some(String::from(customer));
    match features::Book::repositoy::create(&book, pool).await{
        Ok(book) => Ok(Json(book)),
        Err(_) => Err(CustomError::BadRequest(String::from("book already exist")))
    }
}
#[get("/")]
pub async fn create_html() -> content::RawHtml<String> {
    RawHtml(r#"
          <form class="generated-form"  method="POST" action="http:\\localhost:8000/books"  target="_self">
<fieldset>
  <legend></legend>
  <label for="name">First name:</label><br>
  <input type="text"  name="name" value="John"><br>
  <label for="customer_id">customer_id:</label><br>
  <input type="text"  name="customer_id" value="Doe"><br>
  <label for="visibilityMode_id">visibilityMode_id:</label><br>
  <input type="text"  name="visibilityMode_id" value="youremail@gmail.com"><br><br>
  <input type="submit" value="Submit">
</fieldset>
</form>
    "#.to_string())
}
