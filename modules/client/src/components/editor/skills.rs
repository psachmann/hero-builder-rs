use leptos::*;

use super::editor_params::EditorParams;
use crate::{
    components::editor::attributes::Counter,
    i18n::{t, use_i18n, use_i18n_scoped},
    utils::{use_hero_with_id, use_param, ParamUuid},
};

#[component]
pub fn skills() -> impl IntoView {
    let i18n = use_i18n_scoped!(editor.skills);
    let id = use_param::<EditorParams, ParamUuid>(None, |params| params.id);
    let (hero, _) = use_hero_with_id(id);

    view! {
        <div class="container mx-auto flex flex-col gap-2 bg-slate-100 rounded-xl shadow-md p-4">
            <p class="text-black text-xl font-semibold">{t!(i18n, title)}</p>
            <div class="border-2 rounded-xl">
                <table class="table-auto mt-2 w-full">
                    <thead>
                        <tr class="border-b-2">
                            <th class="text-start">{{ t!(i18n, title) }}</th>
                            <th class="text-center">{{ t!(i18n, value) }}</th>
                            <th class="text-center">{{ t!(i18n, trained) }}</th>
                            <th class="text-center">{{ t!(i18n, modifier) }}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {hero
                            .get()
                            .skills
                            .into_iter()
                            .map(|(name, skill)| {
                                view! {
                                    <tr class="border-b-2 last:border-0 hover:bg-gray-200">
                                        <td class="text-start">{name}</td>
                                        <td class="text-center">
                                            <p class="font-semibold">{skill.value}</p>
                                        </td>
                                        <td class="text-center">
                                            <Counter count=skill.trained />
                                        </td>
                                        <td class="text-center">
                                            <Counter count=skill.modifier />
                                        </td>
                                    </tr>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}
