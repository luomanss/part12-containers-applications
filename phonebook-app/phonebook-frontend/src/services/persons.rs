use std::cell::RefCell;

use crate::model::{NewPerson, Person};

thread_local! {
    static PERSONS: RefCell<Vec<Person>> = RefCell::new(vec![
        Person {
            id: "1".to_string(),
            name: "John Doe".to_string(),
            number: "1234567890".to_string(),
        },
        Person {
            id: "2".to_string(),
            name: "Jane Doe".to_string(),
            number: "0987654321".to_string(),
        },
    ]);
}

pub async fn get_all() -> Vec<Person> {
    // reqwest::get("http://localhost:3000/persons")
    //     .await
    //     .unwrap()
    //     .json()
    //     .await
    //     .unwrap()

    PERSONS.with(|persons| persons.borrow().clone())
}

pub async fn remove(id: &str) {
    // reqwest::delete(&format!("http://localhost:3000/persons/{}", id))
    //     .await
    //     .unwrap()

    PERSONS.with(|persons| {
        let mut persons = persons.borrow_mut();

        persons.retain(|person| person.id != id);
    });
}

pub async fn add(person: NewPerson) {
    // reqwest::post("http://localhost:3000/persons")
    //     .json(&person)
    //     .send()
    //     .await
    //     .unwrap()

    PERSONS.with(|persons| {
        let mut persons = persons.borrow_mut();
        let person = Person {
            id: (persons.len() + 1).to_string(),
            name: person.name,
            number: person.number,
        };

        persons.push(person);
    });
}

pub async fn update(id: &str, NewPerson { name, number }: NewPerson) {
    // reqwest::put(&format!("http://localhost:3000/persons/{}", id))
    //     .json(&person)
    //     .send()
    //     .await
    //     .unwrap()

    PERSONS.with(|persons| {
        let mut persons = persons.borrow_mut();

        persons.iter_mut().find(|person| person.id == id).map(|person| {
            person.name = name;
            person.number = number;
        });
    });
}
