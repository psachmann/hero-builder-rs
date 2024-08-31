use leptos::*;

use crate::{
    i18n::{t, use_i18n},
    utils::{use_hero_with_id, use_param, ParamUuid},
};

use super::editor_params::EditorParams;

#[component]
pub fn attributes() -> impl IntoView {
    let i18n = use_i18n();
    let id = use_param::<EditorParams, ParamUuid>(None, |params| params.id);
    let (hero, _) = use_hero_with_id(id);

    view! {
        <div class="container mx-auto bg-slate-100 rounded-lg shadow-md items-center p-4">
            <h1>{t!(i18n, attributes.title)}</h1>
            <p>{t!(i18n, attributes.description)}</p>
            <span />
        </div>
    }
}
