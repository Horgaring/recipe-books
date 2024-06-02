use sqlx::{Error, Pool};
use sqlx_postgres::Postgres;
use crate::domain::entity::customer::Customer;

pub async fn create(customer: &Customer,pool: &Pool<Postgres>) -> Result<Customer,Error> {
    let customer = sqlx::query_as("INSERT INTO Customer (id ,name, email,image_url) values ($1,$2,$3,$4) RETURNING *")
            .bind(&customer.id)
            .bind(&customer.name)
            .bind(&customer.email)
            .bind(&customer.image_url)
            .fetch_one(pool)
            .await;
    customer
}



pub(crate) async fn get_by_id(id: String, pool: &Pool<Postgres>) -> Result<Customer, Error> {
    let res = sqlx::query_as::<Postgres,Customer>("SELECT * FROM Customer WHERE id = $1")
        .bind(id.to_string())
        .fetch_one(pool)
        .await;
    res
}

