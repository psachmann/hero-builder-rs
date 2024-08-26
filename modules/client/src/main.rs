use leptos::*;

use hero_builder_client::App;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Trace).expect("could not initialize logger");

    mount_to_body(|| {
        view! { <App /> }
    });
}
