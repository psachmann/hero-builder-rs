use leptos::*;
use leptos_router::*;

use super::layout::SidebarState;
use crate::{
    components::{editor, home::Home, layout::Layout, not_found::NotFound},
    i18n::provide_i18n_context,
    models::Hero,
};

#[component]
fn placeholder(title: &'static str) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center h-full">
            <h1 class="text-4xl font-bold text-gray-700">{title}</h1>
        </div>
    }
}

#[component]
pub fn app() -> impl IntoView {
    provide_i18n_context();
    provide_context(create_rw_signal(AppState::default()));

    view! {
        <Router>
            <Layout>
                <Routes>
                    <Route path="/" view=Home />
                    <Route path="/editor/:id" view=editor::Index>
                        <Route path="" view=|| view! { <Placeholder title="Select a Hero!" /> } />
                        <Route path="metadata" view=editor::Metadata />
                        <Route path="attributes" view=editor::Attributes />
                        <Route path="skills" view=|| view! { <Placeholder title="Skills" /> } />
                        <Route path="powers" view=|| view! { <Placeholder title="Powers" /> } />
                        <Route
                            path="equipment"
                            view=|| view! { <Placeholder title="Equipment" /> }
                        />
                    </Route>
                    <Route path="/*any" view=NotFound />
                </Routes>
            </Layout>
        </Router>
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub heros: Vec<Hero>,
    pub sidebar: SidebarState,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            heros: vec![Hero::default()],
            sidebar: SidebarState::default(),
        }
    }
}
