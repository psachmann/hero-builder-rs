use leptos::*;
use leptos_router::*;

use crate::i18n::{t, use_i18n};

#[component]
pub fn footer() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <div class="container mx-auto bg-slate-700 text-white py-2 mt-4 rounded-t-xl">
            <div class="container mx-auto flex flex-col md:flex-row justify-center">
                <div class="flex space-x-4 mt-2 md:mt-0">
                    <span>{t!(i18n, footer.copyright)}</span>
                    <A
                        href="https://github.com/psachmann/hero-builder-rs"
                        class="text-blue-400 hover:underline"
                        target="tab"
                    >
                        {t!(i18n, footer.docs)}
                    </A>
                </div>
            </div>
        </div>
    }
}
