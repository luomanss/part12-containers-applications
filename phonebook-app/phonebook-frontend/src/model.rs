use serde::Deserialize;

#[derive(Clone)]
pub struct NewPerson {
    pub name: String,
    pub number: String,
}

#[derive(Deserialize, Clone)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub number: String,
}
