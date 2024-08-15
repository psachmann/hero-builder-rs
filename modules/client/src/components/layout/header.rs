use leptos::*;

#[component]
pub fn header() -> impl IntoView {
    view! {
        <div class="container mx-auto rounded-b-2xl pt-2 mb-4 bg-slate-400">
            <h1>{"Header"}</h1>
        </div>
    }
}
