use uuid::Uuid;

pub trait Entity {
    fn get_id(&self) ->  &Uuid;
}