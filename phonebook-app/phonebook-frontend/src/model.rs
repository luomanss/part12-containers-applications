use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct NewPerson {
    pub name: String,
    pub number: String,
}

#[derive(Deserialize, Clone)]
pub struct Person {
    pub id: u32,
    pub name: String,
    pub number: String,
}
