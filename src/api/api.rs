use rocket::Route;
use rocket::routes;
use crate::features;


pub fn book_endpoints() -> Vec<Route> {
    routes![features::Book::routes::create,
        features::Book::routes::create_html]
}
pub fn recipe_endpoints() -> Vec<Route> {
    routes![features::Recipe::routes::create]
}

pub fn customer_endpoints() -> Vec<Route> {
    routes![features::Customer::routes::get_by_id_endpoint]
}
pub fn topic_endpoints() -> Vec<Route> {
    routes![features::Topic::routes::create_endpoint]
}
