use futures::channel::oneshot;
use gloo_utils::format::JsValueSerdeExt;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{Request, RequestInit, Response};

use crate::model::{NewPerson, Person};

const BACKEND_URL: Option<&str> = option_env!("BACKEND_URL");

pub async fn get_all() -> Vec<Person> {
    let opts = RequestInit::new();
    let url = format!("{}/persons", BACKEND_URL.unwrap_or("http://localhost:3000"));

    opts.set_method("GET");

    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    let window = web_sys::window().unwrap();
    let response: Response = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap()
        .dyn_into()
        .unwrap();

    let response = JsFuture::from(response.json().unwrap()).await.unwrap();

    response.into_serde().unwrap()
}

pub async fn remove(id: u32) {
    let (send, recv) = oneshot::channel();

    spawn_local(async move {
        let opts = RequestInit::new();
        let url = format!(
            "{}/persons/{}",
            BACKEND_URL.unwrap_or("http://localhost:3000"),
            id
        );

        opts.set_method("DELETE");

        let request = Request::new_with_str_and_init(&url, &opts).unwrap();
        let window = web_sys::window().unwrap();
        let _ = JsFuture::from(window.fetch_with_request(&request)).await;

        send.send(()).unwrap();
    });

    let _ = recv.await;
}

pub async fn add(person: NewPerson) {
    let (send, recv) = oneshot::channel();

    spawn_local(async move {
        let opts = RequestInit::new();
        let url = format!("{}/persons", BACKEND_URL.unwrap_or("http://localhost:3000"));

        opts.set_method("POST");
        opts.set_body(&JsValue::from_str(
            serde_json::to_string(&person).unwrap().as_str(),
        ));

        let request = Request::new_with_str_and_init(&url, &opts).unwrap();

        request
            .headers()
            .set("content-type", "application/json")
            .unwrap();

        let window = web_sys::window().unwrap();
        let _ = JsFuture::from(window.fetch_with_request(&request)).await;

        send.send(()).unwrap();
    });

    let _ = recv.await;
}

pub async fn update(id: u32, person: NewPerson) {
    let (send, recv) = oneshot::channel();

    spawn_local(async move {
        let opts = RequestInit::new();
        let url = format!(
            "{}/persons/{}",
            BACKEND_URL.unwrap_or("http://localhost:3000"),
            id
        );

        opts.set_method("PATCH");
        opts.set_body(&JsValue::from_serde(&person).unwrap());

        let request = Request::new_with_str_and_init(&url, &opts).unwrap();
        let window = web_sys::window().unwrap();
        let _ = JsFuture::from(window.fetch_with_request(&request)).await;

        send.send(()).unwrap();
    });

    let _ = recv.await;
}
