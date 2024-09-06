use leptos::*;

use super::editor_params::EditorParams;
use crate::{
    i18n::{t, use_i18n, use_i18n_scoped},
    utils::{use_hero_with_id, use_param, ParamUuid},
};

// TODO: This should be a shared component
#[component]
pub fn counter(count: RwSignal<i32>) -> impl IntoView {
    let decrement = move || count.update(|v| *v -= 1);
    let increment = move || count.update(|v| *v += 1);

    view! {
        <div class="inline-flex flex-row bg-blue-400 rounded-full justify-center items-center">
            <p
                class="text-xl font-semibold px-2 hover:cursor-pointer"
                on:click=move |_| { decrement() }
            >
                {"-"}
            </p>
            <p class="text-center font-semibold bg-slate-200 rounded-full w-6 h-6">{count}</p>
            <p
                class="text-xl font-semibold px-2 hover:cursor-pointer"
                on:click=move |_| { increment() }
            >
                {"+"}
            </p>
        </div>
    }
}

#[component]
pub fn attributes() -> impl IntoView {
    let i18n_attributes = use_i18n_scoped!(editor.attributes);
    let i18n_derived_attributes = use_i18n_scoped!(editor.derived_attributes);
    let id = use_param::<EditorParams, ParamUuid>(None, |params| params.id);
    let (hero, _) = use_hero_with_id(id);

    view! {
        <div class="container mx-auto flex flex-col gap-2 bg-slate-100 rounded-xl shadow-md p-4">
            <p class="text-black text-xl font-semibold">{t!(i18n_attributes, title)}</p>
            <div class="border-2 rounded-xl">
                <table class="table-auto mt-2 w-full">
                    <thead>
                        <tr class="border-b-2">
                            <th class="text-start">{{ t!(i18n_attributes, title) }}</th>
                            <th class="text-center">{{ t!(i18n_attributes, value) }}</th>
                            <th class="text-center">{{ t!(i18n_attributes, trained) }}</th>
                            <th class="text-center">{{ t!(i18n_attributes, initial) }}</th>
                            <th class="text-center">{{ t!(i18n_attributes, modifier) }}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {hero
                            .get()
                            .attributes
                            .into_iter()
                            .map(|(idx, attribute)| {
                                view! {
                                    <tr class="border-b-2 last:border-0 hover:bg-gray-200">
                                        <td class="text-start">
                                            {{ t!(i18n_attributes, names, count = move || idx) }}
                                        </td>
                                        <td class="text-center">
                                            <p class="font-semibold">{attribute.value}</p>
                                        </td>
                                        <td class="text-center">
                                            <Counter count=attribute.initial />
                                        </td>
                                        <td class="text-center">
                                            <Counter count=attribute.trained />
                                        </td>
                                        <td class="text-center">
                                            <Counter count=attribute.modifier />
                                        </td>
                                    </tr>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </tbody>
                </table>
            </div>
            <p class="text-black text-xl font-semibold pt-4">
                {t!(i18n_derived_attributes, title)}
            </p>
            <div class="border-2 rounded-xl">
                <table class="table-auto mt-2 w-full">
                    <thead>
                        <tr class="border-b-2">
                            <th class="text-start">{{ t!(i18n_derived_attributes, title) }}</th>
                            <th class="text-center">{{ t!(i18n_derived_attributes, value) }}</th>
                            <th class="text-center">{{ t!(i18n_derived_attributes, modifier) }}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {hero
                            .get()
                            .derived_attributes
                            .into_iter()
                            .map(|(idx, attribute)| {
                                view! {
                                    <tr class="border-b-2 last:border-0 hover:bg-gray-200">
                                        <td class="text-start">
                                            {{
                                                t!(i18n_derived_attributes, names, count = move || idx)
                                            }}
                                        </td>
                                        <td class="text-center">
                                            <p class="font-semibold">{attribute.value}</p>
                                        </td>
                                        <td class="text-center">
                                            <Counter count=attribute.modifier />
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
