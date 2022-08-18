use serde::{
    Serialize,
    Deserialize
};

#[derive(Serialize)]
pub struct Domain {
    pub id: uuid::Uuid,
    pub active: bool,
    pub name: String,
    pub slug: String
}