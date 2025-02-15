mod app;
pub mod services;
pub mod components;
pub mod util;
mod model;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init().expect("could not initialize logger");
    leptos::mount::mount_to_body(App)
}
