use leptos::*;
use leptos_router::*;

use crate::{
    components::not_found::NotFound, models::Hero, utils::{use_app_state, use_param, ParamUuid}
};

#[component]
pub fn editor() -> impl IntoView {
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
        <Show when=move || hero.get().is_some() fallback=|| view! { <NotFound /> }>
            <div class="flex flex-col gap-4">
                <div class="container mx-auto bg-slate-100 rounded-lg shadow-md items-center">
                    <div class="flex flex-row justify-center items-center p-4">
                        <A href="metadata" class="flex-1 text-blue-400 hover:underline">
                            {"Metadata"}
                        </A>
                        <A href="attributes" class="flex-1 text-blue-400 hover:underline">
                            {"Attributes"}
                        </A>
                        <A href="skills" class="flex-1 text-blue-400 hover:underline">
                            {"Skills"}
                        </A>
                        <A href="powers" class="flex-1 text-blue-400 hover:underline">
                            {"Powers"}
                        </A>
                        <A href="equipment" class="flex-1 text-blue-400 hover:underline">
                            {"Equipment"}
                        </A>
                    </div>
                </div>
                <Outlet />
            </div>
        </Show>
    }
}

#[derive(PartialEq, Params)]
pub struct EditorParams {
    /// The UUID of the hero to edit.
    id: ParamUuid,
}

#[component]
pub fn metadata(hero: RwSignal<Option<Hero>>) -> impl IntoView {
    view! {
        <div class="container mx-auto bg-slate-100 rounded-lg shadow-md">
            <div class="flex flex-col m-4 items-center">
                <div class="flex flex-row w-full items-center pb-2 border-b-2">
                    <p class="basis-1/3 block text-gray-900 text-sm font-bold mb-2">Metadata</p>

                </div>
                <div class="flex flex-row w-full items-center py-4 border-b-2">
                    <label class="basis-1/3 block text-gray-700 text-sm font-bold mb-2" for="name">
                        Name
                    </label>
                    <input
                        class="basis-1/2 shadow appearance-none border rounded py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                        id="name"
                        type="text"
                        placeholder="Name"
                        value=hero.get().map(|hero| hero.name).unwrap_or_default()
                    />
                </div>
                <div class="flex flex-row w-full items-center py-4 border-b-2">
                    <label
                        class="basis-1/3 block text-gray-700 text-sm font-bold mb-2"
                        for="hero_rank"
                    >
                        Hero Rank
                    </label>
                    <input
                        class="basis-1/2 shadow appearance-none border rounded py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                        id="hero_rank"
                        type="number"
                        placeholder="Hero Rank"
                        value=hero.get().map(|hero| hero.hero_rank.to_string()).unwrap_or_default()
                    />
                </div>
                <div class="flex flex-row w-full items-center py-4 border-b-2">
                    <label
                        class="basis-1/3 block text-gray-700 text-sm font-bold mb-2"
                        for="size_class"
                    >
                        Size Class
                    </label>
                    <input
                        class="basis-1/2 shadow appearance-none border rounded py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                        id="size_class"
                        type="number"
                        placeholder="Size Class"
                        value=hero.get().map(|hero| hero.size_class.to_string()).unwrap_or_default()
                    />
                </div>
                <div class="flex flex-row w-full items-center py-4 border-b-2">
                    <label
                        class="basis-1/3 block text-gray-700 text-sm font-bold mb-2"
                        for="available_xp"
                    >
                        Available XP
                    </label>
                    <input
                        class="basis-1/2 shadow appearance-none border rounded py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                        id="available_xp"
                        type="number"
                        placeholder="Available XP"
                        value=hero
                            .get()
                            .map(|hero| hero.available_xp.to_string())
                            .unwrap_or_default()
                    />
                </div>
                <div class="flex flex-row w-full items-center py-4 border-b-2">
                    <label
                        class="basis-1/3 block text-gray-700 text-sm font-bold mb-2"
                        for="spent_xp"
                    >
                        Spent XP
                    </label>
                    <input
                        class="basis-1/2 shadow appearance-none border rounded py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                        id="spent_xp"
                        type="number"
                        placeholder="Spent XP"
                        value=hero.get().map(|hero| hero.spent_xp.to_string()).unwrap_or_default()
                    />
                </div>
                <span />
                <div class="flex flex-row w-full items-center py-4 border-b-2">
                    <p class="basis-1/3 block text-gray-900 text-sm font-bold mb-2">Attributes</p>
                </div>
            </div>
        </div>
    }
}
