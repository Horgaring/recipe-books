use rocket::Route;
use rocket::routes;
use crate::features::Customer::get_by_id::get_by_id_endpoint;
use crate::features::Topic::get_by_name;


pub fn book_endpoints() -> Vec<Route> {
    routes![]
}
pub fn recipe_endpoints() -> Vec<Route> {
    vec![]
}
pub fn library_endpoints() -> Vec<Route> {
    vec![]
    
}
pub fn customer_endpoints() -> Vec<Route> {
    routes![get_by_id_endpoint]
}
pub fn topic_endpoints() -> Vec<Route> {
    routes![get_by_name::get_by_name_endpoint]
}
