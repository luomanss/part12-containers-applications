use leptos::prelude::*;

#[component]
pub fn Filter(filter: ReadSignal<String>, set_filter: WriteSignal<String>) -> impl IntoView {
    view! {
        <div>
            {"filter shown with "}
            <input
                type="text"
                on:input:target=move |ev| { set_filter.set(ev.target().value()) }
                prop:value=filter
            />
        </div>
    }
}