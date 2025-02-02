mod app;
pub mod services;
pub mod components;
pub mod util;
mod model;

use app::App;

fn main() {
    dotenv::dotenv().ok();
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
