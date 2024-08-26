use leptos::*;
use leptos_i18n::*;
use leptos_router::*;

use crate::i18n;

#[component]
pub fn home() -> impl IntoView {
    let i18n = i18n::use_i18n();

    view! {
        <div class="container mx-auto flex items-center justify-center p-4">
            <div class="flex flex-col md:flex-row items-center bg-slate-50 rounded-2xl shadow-2xl overflow-hidden">
                <div class="p-8 md:w-1/2">
                    <div>
                        <h1 class="text-4xl font-bold text-black mb-4">
                            {t!(i18n, home.welcome_title)}
                        </h1>
                        <p class="text-gray-700">{t!(i18n, home.welcome_text)}</p>
                    </div>
                    <div class="text-center mt-6">
                        <A
                            href="/counter"
                            class="bg-blue-500 text-white font-bold py-2 px-4 rounded-lg hover:bg-blue-700"
                        >
                            {t!(i18n, home.start_button)}
                        </A>
                    </div>
                </div>
                <div class="md:w-1/2">
                    <img
                        src="./imgs/start_your_hero_journey.jfif"
                        alt="Hero Image"
                        class="w-full h-auto rounded-r-lg"
                    />
                </div>
            </div>
        </div>
    }
}
