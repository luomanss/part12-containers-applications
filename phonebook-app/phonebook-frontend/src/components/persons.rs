use crate::{
    app::GetAllPersons, components::Notification, model::Person, services::persons::remove,
    util::WakerSender,
};
use leptos::prelude::*;

#[component]
pub fn Persons(persons: Signal<Vec<Person>>, filter: ReadSignal<String>) -> impl IntoView {
    let waker = use_context::<WakerSender<GetAllPersons>>().unwrap();
    let notify = use_context::<Action<Notification, ()>>().unwrap();

    let filteredPersons = move || {
        persons
            .get()
            .into_iter()
            .filter(|person| {
                person
                    .name
                    .to_lowercase()
                    .contains(&filter.get().to_lowercase())
            })
            .collect::<Vec<_>>()
    };

    let delete = Action::new(move |person: &Person| {
        let person = person.clone();

        async move {
            remove(&person.id).await;
            waker.wake();
            notify.dispatch(Notification::success(
                format!("Deleted {}", person.name).as_str(),
            ));
        }
    });

    view! {
        <div>
            <For
                each=move || filteredPersons()
                key=|person| person.id.clone()
                children=move |person| {
                    view! {
                        <div>
                            {format!("{} {}", person.name, person.number)}
                            <button on:click=move |ev| {
                                ev.prevent_default();
                                delete.dispatch(person.clone());
                            }>delete</button>
                        </div>
                    }
                }
            />
        </div>
    }
}
