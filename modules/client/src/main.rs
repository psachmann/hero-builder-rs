use leptos::*;

fn main() {
    _ = console_error_panic_hook::set_once();

    mount_to_body(|| view! {
        <div class="flex justify-center">
            <p class="text-4xl font-bold pt-16">{ "Hello, world from Leptos!" }</p>
        </div>
    });
}
