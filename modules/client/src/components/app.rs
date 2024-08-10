use leptos::*;

#[component]
pub fn app() -> impl IntoView {
    view! {
        <div class="flex justify-center">
            <p class="text-4xl font-bold pt-16">{ "Hello, world from Leptos!" }</p>
        </div>
    }
}
