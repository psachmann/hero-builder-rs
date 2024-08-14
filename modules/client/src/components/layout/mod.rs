mod footer;
mod header;
mod sidebar;

use leptos::*;

#[component]
pub fn layout(children: ChildrenFn) -> impl IntoView {
    view! {
        <div>
            <header />
            <main>
                <div>
                    {children()}
                </div>
            </main>
            <footer />
        </div>
    }
}
