use leptos::*;

use crate::{
    i18n::{t, use_i18n}, models::Hero, utils::{use_hero_with_id, use_param, ParamUuid}
};

use super::editor_params::EditorParams;

#[component]
pub fn counter(count: RwSignal<i32>) -> impl IntoView {
    let decrement = move || count.update(|v| *v -= 1);
    let increment = move || count.update(|v| *v += 1);

    view! {
        <div class="inline-flex flex-row bg-blue-400 rounded-full justify-center items-center">
            <p class="text-xl font-semibold px-2 hover:cursor-pointer" on:click=move |_| { decrement() }>{"-"}</p>
            <p class="text-center font-semibold bg-slate-200 rounded-full w-6 h-6">{count}</p>
            <p class="text-xl font-semibold px-2 hover:cursor-pointer" on:click=move |_| { increment() }>{"+"}</p>
        </div>
    }
}

#[component]
pub fn attributes() -> impl IntoView {
    let i18n = use_i18n();
    let id = use_param::<EditorParams, ParamUuid>(None, |params| params.id);
    let (hero, set_hero) = use_hero_with_id(id);

    view! {
        <div class="container mx-auto flex flex-col gap-2 bg-slate-100 rounded-lg shadow-md p-4">
            <p class="text-black text-xl font-semibold">{t!(i18n, editor.attributes.title)}</p>
            <div class="border-2 rounded-lg">
            <table class="table-auto w-full">
                <thead>
                    <tr class="border-b-2">
                        <th class="text-start">{{t!(i18n, editor.attributes.title)}}</th>
                        <th class="text-center">{{t!(i18n, editor.attributes.value)}}</th>
                        <th class="text-center">{{t!(i18n, editor.attributes.trained)}}</th>
                        <th class="text-center">{{t!(i18n, editor.attributes.initial)}}</th>
                        <th class="text-center">{{t!(i18n, editor.attributes.modifier)}}</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        hero.get().attributes.into_iter().map(|(name, value)| {
                            let current = move || value.0.get() + value.1.get() + value.2.get();

                            view! {
                                <tr class="border-b-2 last:border-0 hover:bg-gray-200">
                                    <td class="text-start">{name}</td>
                                    <td class="text-center">
                                        <p class="font-semibold">{current}</p>
                                    </td>
                                    <td class="text-center">
                                        <Counter count={value.0.clone()} />
                                    </td>
                                    <td class="text-center">
                                        <Counter count={value.1.clone()} />
                                    </td>
                                    <td class="text-center">
                                        <Counter count={value.2.clone()} />
                                    </td>
                                </tr>
                            }
                        }).collect::<Vec<_>>()
                    }
                </tbody>
            </table>
            </div>
            <p class="text-black text-xl font-semibold pt-4">{t!(i18n, editor.derived_attributes.title)}</p>
            <table class="table-auto border-2 rounded-lg w-full">
            <thead>
                    <tr>
                        <th class="text-start">{{t!(i18n, editor.attributes.title)}}</th>
                        <th class="text-start">{{t!(i18n, editor.attributes.value)}}</th>
                        <th class="text-start">{{t!(i18n, editor.attributes.trained)}}</th>
                        <th class="text-start">{{t!(i18n, editor.attributes.initial)}}</th>
                        <th class="text-start">{{t!(i18n, editor.attributes.modifier)}}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>{"Charisma"}</td>
                        <td>{"4"}</td>
                        <td>{"3"}</td>
                        <td>{"1"}</td>
                        <td>{"0"}</td>
                    </tr>
                    <tr>
                        <td>{"Intelligenz"}</td>
                        <td>{"4"}</td>
                        <td>{"3"}</td>
                        <td>{"1"}</td>
                        <td>{"0"}</td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}
