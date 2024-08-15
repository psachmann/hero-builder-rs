use leptos::*;

use hero_builder_client::components::App;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { <App /> }
    });
}
