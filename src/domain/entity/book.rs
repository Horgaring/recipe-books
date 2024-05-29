use uuid::Uuid;


pub struct Book {
    pub title: String,
    pub pages: u32,
    pub customer_id: Uuid,
}

