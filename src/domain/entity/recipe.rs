use crate::domain::entity::common::ingredient::Ingredient;

pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<Ingredient>,
    pub instruction: String,
    pub topic_id: uuid::Uuid,
}