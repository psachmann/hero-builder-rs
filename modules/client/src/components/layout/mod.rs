mod footer;
mod header;
mod sidebar;

use leptos::*;

#[component]
pub fn layout(children: ChildrenFn) -> impl IntoView {
    view! {
        <div class="grid grid-cols-10 grid-rows-3 gap-2 bg-slate-400">
            <header class="col-span-10 row-span-1 bg-slate-200">
                <h1>{"Header"}</h1>
            </header>
            <aside class="col-span-2 row-span-1 bg-slate-200">
                <h1>{"Sidebar"}</h1>
            </aside>
            <main class="col-span-8 row-span-1 bg-slate-200">
                <div>
                    {children()}
                </div>
            </main>
            <footer class="col-span-10 row-span-1 bg-slate-200">
                <h1>{"Footer"}</h1>
            </footer>
        </div>
    }
}
