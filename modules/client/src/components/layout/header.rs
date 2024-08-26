use leptos::*;
use leptos_router::*;
use log::info;

use crate::i18n::{self, t, use_i18n};

#[component]
pub fn header() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <div class="container mx-auto bg-slate-900 text-white p-4 rounded-b-2xl">
            <nav class="container mx-auto flex flex-col md:flex-row justify-between items-center">
                <div class="flex flex-grow justify-center space-x-4 mt-2 md:mt-0">
                    <A href="/" class="text-blue-400 hover:underline">
                        {t!(i18n, header.title)}
                    </A>
                </div>
                <div class="mt-2 md:mt-0">
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
            </nav>
        </div>
    }
}
