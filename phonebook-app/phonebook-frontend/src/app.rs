use leptos::prelude::*;
use std::{future::IntoFuture, time::Duration};

use crate::{
    components::{Filter, Notification, PersonForm, Persons},
    services::persons::get_all,
    util::{sleep, use_waker},
};

pub struct GetAllPersons;

#[component]
pub fn App() -> impl IntoView {
    let (filter, set_filter) = signal(String::new());
    let (notification, set_notification) = signal(None);
    let waker = use_waker::<GetAllPersons>();

    let persons = LocalResource::new(move || {
        waker.subscribe();

        async move { get_all().await }
    });

    let show_notification = Action::new(move |input: &Notification| {
        let input = input.clone();

        async move {
            set_notification.set(Some(input));
            sleep(Duration::from_millis(2000)).await;
            set_notification.set(None);
        }
    });

    provide_context(show_notification);

    view! {
        <div>
            <h2>Phonebook</h2>
            <Notification notification=notification />
            <Filter filter=filter set_filter=set_filter />
            <h2>add a new</h2>
            <PersonForm />
            <Suspense>
              { move || Suspend::new(async move {
                let persons = persons.into_future().await;

                view! { <Persons persons=Signal::stored(persons) filter=filter /> }
              })
            }
            </Suspense>
        </div>
    }
}
