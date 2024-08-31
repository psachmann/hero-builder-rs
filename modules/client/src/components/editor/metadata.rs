use leptos::*;

use super::editor_params::EditorParams;
use crate::utils::{use_hero_with_id, use_param, ParamUuid};

#[component]
pub fn metadata() -> impl IntoView {
    let id = use_param::<EditorParams, ParamUuid>(None, |params| params.id);
    let (hero, _) = use_hero_with_id(id);

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
                        value=hero.get().name
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
                        value=hero.get().hero_rank.to_string()
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
                        value=hero.get().size_class.to_string()
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
                        value=hero.get().available_xp.to_string()
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
                        value=hero.get().spent_xp.to_string()
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
