use leptos::*;

use crate::i18n::{self, t, use_i18n};

#[component]
pub fn sidebar() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <div class="flex flex-col text-white m-4">
            <select
                class="bg-slate-800 text-white py-1 px-2 rounded"
                on:change=move |ev| {
                    let new_locale = match event_target_value(&ev).as_str() {
                        "en" => i18n::Locale::en,
                        "de" => i18n::Locale::de,
                        _ => i18n::Locale::en,
                    };
                    i18n.set_locale(new_locale);
                }
            >
                <option value="de">{t!(i18n, header.german)}</option>
                <option value="en">{t!(i18n, header.english)}</option>
            </select>
        </div>
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SidebarState {
    pub is_open: bool,
    pub width: f32,
}

impl Default for SidebarState {
    fn default() -> Self {
        Self {
            is_open: false,
            width: 200.0,
        }
    }
}
