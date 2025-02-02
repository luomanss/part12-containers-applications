use leptos::{ev::SubmitEvent, prelude::*};

use crate::{app::GetAllPersons, model::NewPerson, services::persons::add, util::WakerSender};

#[component]
pub fn PersonForm() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (number, set_number) = signal(String::new());
    let waker = use_context::<WakerSender<GetAllPersons>>().unwrap();

    let add_person = Action::new(move |person: &NewPerson| {
        let person = person.clone();

        async move {
            add(person).await;
            waker.wake();
        }
    });

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let name = name.get();
        let number = number.get();

        if name.is_empty() || number.is_empty() {
            return;
        }

        let person = NewPerson { name, number };

        add_person.dispatch(person);
    };

    view! {
        <form on:submit=on_submit>
            <div>
                {"name: "}
                <input
                    type="text"
                    placeholder="Name"
                    on:input:target=move |ev| { set_name.set(ev.target().value()) }
                    prop:value=name
                />
            </div>
            <div>
                {"number: "}
                <input
                    type="text"
                    placeholder="Number"
                    on:input:target=move |ev| { set_number.set(ev.target().value()) }
                    prop:value=number
                />
            </div>
            <div>
                <button type="submit">Add</button>
            </div>
        </form>
    }
}
