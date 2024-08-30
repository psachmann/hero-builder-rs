use leptos::*;
use leptos_router::*;
use log::info;

use crate::{
    i18n::{self, t, use_i18n},
    utils::use_app_state,
};

#[component]
pub fn menu_button() -> impl IntoView {
    let state = use_app_state();
    let (is_open, set_is_open) = create_slice(
        state,
        |state| state.sidebar.is_open,
        |state, v| state.sidebar.is_open = v,
    );

    view! {
        <button
            class="flex flex-col space-y-1 py-2"
            on:click=move |_| {
                info!("Toggling sidebar");
                set_is_open.set(!is_open.get())
            }
        >
            <span class=move || {
                if is_open.get() {
                    "block w-6 h-0.5 bg-white transform rotate-45 translate-y-1.5 transition duration-300"
                } else {
                    "block w-6 h-0.5 bg-white transition duration-300"
                }
            }></span>
            <span class=move || {
                if is_open.get() {
                    "block w-6 h-0.5 bg-white opacity-0 transition duration-300"
                } else {
                    "block w-6 h-0.5 bg-white transition duration-300"
                }
            }></span>
            <span class=move || {
                if is_open.get() {
                    "block w-6 h-0.5 bg-white transform -rotate-45 -translate-y-1.5 transition duration-300"
                } else {
                    "block w-6 h-0.5 bg-white transition duration-300"
                }
            }></span>
        </button>
    }
}

#[component]
pub fn header() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <div class="flex flex-row text-white p-4">
            <div>
                <MenuButton />
            </div>
            <div class="flex flex-grow justify-center items-center">
                <A href="/" class="text-blue-400 hover:underline">
                    {t!(i18n, header.title)}
                </A>
            </div>
            <div>
                <select
                    class="bg-slate-800 text-white py-1 px-2 rounded"
                    on:change=move |ev| {
                        let new_locale = match event_target_value(&ev).as_str() {
                            "en" => i18n::Locale::en,
                            "de" => i18n::Locale::de,
                            _ => i18n::Locale::en,
                        };
                        info!("Setting locale to {:?}", new_locale);
                        i18n.set_locale(new_locale);
                    }
                >
                    <option value="de">{t!(i18n, header.german)}</option>
                    <option value="en">{t!(i18n, header.english)}</option>
                </select>
            </div>
        </div>
    }
}
