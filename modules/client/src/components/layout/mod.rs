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
    let sidebar_is_closed = move || !sidebar_is_open.get();

    view! {
        <div class="flex flex-col min-h-screen bg-slate-300">
            <header class="bg-slate-700 ">
                <Header />
            </header>
                <div class="relative flex flex-row flex-grow">
                    <aside class="absolute top-0 left-0 bottom-0 w-1/4 bg-slate-700 my-4 rounded-r-2xl z-10" class=("hidden", sidebar_is_closed)>
                        <Sidebar />
                    </aside>
                    <main class="flex-grow p-4 transition-colors duration-300" class=("bg-gray-500", sidebar_is_open) class=("opacity-50", sidebar_is_open) class=("pointer-events-none", sidebar_is_open) >
                        {children()}
                    </main>
                </div>
            <footer class="transition-colors duration-300" class=("bg-gray-500", sidebar_is_open) class=("opacity-50", sidebar_is_open)>
                <Footer />
            </footer>
        </div>
    }
}
