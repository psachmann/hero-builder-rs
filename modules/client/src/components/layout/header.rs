use leptos::*;

#[component]
pub fn header() -> impl IntoView {
    view! {
        <div class="container mx-auto basis-3/4 bg-slate-400">
            <h1>{"Header"}</h1>
        </div>
    }
}
