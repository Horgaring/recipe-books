use rocket::{get, post, State};
use rocket::form::{Form};
use rocket::http::CookieJar;
use rocket::response::content;
use rocket::response::content::RawHtml;
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
    match features::Book::repositoy::get_by_id(topic.book_id, &pool).await{
        Ok(book) => {
            if book.customer_id.unwrap() != customer {
                return Err(CustomError::PermissionDenied("Customer id is not equal customer id in book entity"));
            }
        }
        Err(_) => {return Err(CustomError::BadRequest(String::from("Book with some id does not exist")));}
    };
    match create(topic.into_inner(), pool.inner()).await {
        Ok(topic) => Ok(Json(topic)),
        Err(err) => Err(CustomError::BadRequest(String::from("Topic Already exist"))),

    }
}
#[get("/")]
pub async fn create_html() -> content::RawHtml<String> {
    RawHtml(r#"
          <form class="generated-form"  method="POST" action="http:\\localhost:8000/topics"  target="_self">
<fieldset>
  <legend></legend>
  <label for="name">First name:</label><br>
  <input type="text"  name="name" value="John"><br>
  <label for="description">description:</label><br>
  <input type="text"  name="description" value="John"><br>
  <label for="book_id">book_id:</label><br>
  <input type="text"  name="book_id" value="John"><br>
  <input type="submit" value="Submit">
</fieldset>
</form>
    "#.to_string())
}
