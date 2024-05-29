use recipe_books::{api::api::{book_endpoints, customer_endpoints, library_endpoints, recipe_endpoints}, persistence::db};
use rocket::{ response::Redirect};
use recipe_books::persistence::db::get_db_pool;
use recipe_books::persistence::auth::google_auth::{authorize, AuthGoogleRequest, callback, external};


#[macro_use] extern crate rocket;


#[launch]
async fn  rocket() -> _ {  
    let pool = get_db_pool().await;

    rocket::build()
        .manage(pool)
        .mount("/books", book_endpoints())
        .mount("/recipes", recipe_endpoints())
        .mount("/customers", customer_endpoints())
        .mount("/libraries", library_endpoints())
        .mount("/auth0", routes![external,callback])

}