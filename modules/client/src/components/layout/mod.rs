mod footer;
mod header;
mod sidebar;

use leptos::*;

use footer::Footer;
use header::Header;
use sidebar::Sidebar;

#[component]
pub fn layout(children: ChildrenFn) -> impl IntoView {
    view! {
        <div class="grid grid-cols-10 auto-rows-auto gap-2 bg-slate-400">
            <header class="col-span-10 row-span-1 bg-slate-200">
                <Header />
            </header>
            <aside class="col-span-2 row-span-1 bg-slate-200">
                <Sidebar />
            </aside>
            <main class="col-span-8 row-span-1 bg-slate-200">
                <div>{children()}</div>
            </main>
            <footer class="col-span-10 row-span-1 bg-slate-200">
                <Footer />
            </footer>
        </div>
    }
}
