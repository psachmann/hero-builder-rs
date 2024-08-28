use leptos::*;
use leptos_router::*;

use crate::i18n::{t, use_i18n};

#[component]
pub fn not_found() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <div class="container mx-auto text-center pt-12">
            <p class="text-4xl font-bold text-black mb-8">{t!(i18n, not_found.title)}</p>
            <span />
            <A
                href="/"
                class="bg-blue-500 text-white font-bold py-2 px-4 rounded-lg hover:bg-blue-700"
            >
                {t!(i18n, not_found.back_to_home)}
            </A>
        </div>
    }
}
