use leptos::*;
use leptos_router::*;

use crate::{components::not_found::NotFound, i18n::{t, use_i18n}, utils::{use_app_state, use_param, ParamUuid}};

#[component]
pub fn editor() -> impl IntoView {
    let i18n = use_i18n();
    let id = use_param::<EditorParams, ParamUuid>(None, |params| params.id);
    let state = use_app_state();
    let hero = create_read_slice(state, move |state| {
        state
            .heros
            .clone()
            .into_iter()
            .find(|hero| hero.id == id.get().into())
    });

    view! {
        <Show
            when=move || hero.get().is_some()
            fallback=|| view! { <NotFound /> }
        >
            <div class="container mx-auto text-center bg-slate-400">
                <p class="text-white text-4xl font-bold p-4">
                    {t!(i18n, hello_world)}
                </p>
            </div>
        </Show>
    }
}

#[derive(PartialEq, Params)]
pub struct EditorParams {
    /// The UUID of the hero to edit.
    id: ParamUuid,
}
