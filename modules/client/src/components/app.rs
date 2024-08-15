use leptos::*;
use leptos_router::*;

use super::layout::Layout;

#[component]
fn counter() -> impl IntoView {
    let counter = create_rw_signal(0);

    view! {
        <div class="container mx-auto">
            <div class="flex flex-row gap-4">
                <div>
                    <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" on:click={ move |_| counter.set(counter.get() - 1) }>
                        { "Decrement" }
                    </button>
                </div>
                <div>
                    <p class="text-lg font-bold pt-16">{ counter }</p>
                </div>
                <div>
                    <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" on:click={ move |_| counter.set(counter.get() + 1) }>
                        { "Increment" }
                    </button>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn app() -> impl IntoView {
    view! {
        <Router>
            <Layout>
                <Routes>
                    <Route path="/" view=Counter />
                </Routes>
            </Layout>
        </Router>
    }
}
