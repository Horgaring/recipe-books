use recipe_books::persistence::auth::google::{ callback, external};
use recipe_books::persistence::auth::jwt::authorize;
use recipe_books::persistence::db::get_db_pool;
use recipe_books::{
    api::api::{book_endpoints, customer_endpoints, recipe_endpoints},

};
use rocket::http::CookieJar;
use recipe_books::api::api::topic_endpoints;
use recipe_books::error::CustomError;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    let pool = get_db_pool().await;
    rocket::build()
        .manage(pool)
        .mount("/books", book_endpoints())
        .mount("/recipes", recipe_endpoints())
        .mount("/customers", customer_endpoints())
        .mount("/topics",topic_endpoints())
        .mount("/auth0", routes![external, callback])
        .mount("/1", routes![test])
}
#[get("/2")]
fn test(cookie: &CookieJar<'_>) -> Result<String,CustomError>{
    match authorize(cookie) {
        Ok(id) => {Ok(id)}
        Err(pa) => {
            println!("{}",pa);
            Err(CustomError::Redirect("/auth0/login".to_string()))}
    }
}