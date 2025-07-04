use leptos::{mount::mount_to_body, prelude::*};

use crate::app::App;

mod app;
mod v5;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}
