mod footer;
mod header;
mod sidebar;

use leptos::*;

pub use sidebar::SidebarState;

use footer::Footer;
use header::Header;
use sidebar::Sidebar;

use crate::utils::use_app_state;

#[component]
pub fn layout(children: ChildrenFn) -> impl IntoView {
    let state = use_app_state();
    let sidebar_is_open = create_read_slice(state, |state| state.sidebar.is_open);

    view! {
        <div class="flex flex-row min-h-screen bg-slate-300">
            <aside class="w-1/4 bg-slate-700 p-4" class=("hidden", move || !sidebar_is_open.get())>
                <Sidebar />
            </aside>
            <div class="flex flex-col w-full">
                <header class="bg-slate-700">
                    <Header />
                </header>
                <main class="flex-grow p-4">{children()}</main>
                <footer>
                    <Footer />
                </footer>
            </div>
        </div>
    }
}
