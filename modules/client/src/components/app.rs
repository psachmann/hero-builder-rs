use leptos::*;
use leptos_router::*;

use super::editor::Editor;
use super::home::Home;
use super::layout::Layout;
use super::not_found::NotFound;
use crate::i18n::provide_i18n_context;
use crate::models::Hero;

#[component]
pub fn app() -> impl IntoView {
    provide_i18n_context();
    provide_context(create_rw_signal(AppState::default()));

    view! {
        <Router>
            <Layout>
                <Routes>
                    <Route path="/" view=Home />
                    <Route path="/editor/:id" view=Editor />
                    <Route path="/*any" view=NotFound />
                </Routes>
            </Layout>
        </Router>
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub heros: Vec<Hero>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            heros: vec![Hero::default()],
        }
    }
}
