mod footer;
mod header;
mod sidebar;

use leptos::*;

use footer::Footer;
use header::Header;

#[component]
pub fn layout(children: ChildrenFn) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2 min-h-screen bg-slate-300">
            <header class="col-span-10 row-span-1">
                <Header />
            </header>
            <main class="flex-grow">{children()}</main>
            <footer>
                <Footer />
            </footer>
        </div>
    }
}
