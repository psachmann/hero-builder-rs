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
            <div class="flex space-x-4">
                <A
                    href="https://github.com/psachmann/hero-builder-rs"
                    class="text-white"
                    target="tab"
                >
                    <svg
                        class="w-6 h-6"
                        fill="currentColor"
                        viewBox="0 0 24 24"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path d="M12 0C5.37 0 0 5.37 0 12c0 5.3 3.438 9.8 8.205 11.385.6.11.82-.26.82-.577 0-.285-.01-1.04-.015-2.04-3.338.725-4.042-1.61-4.042-1.61-.546-1.385-1.333-1.755-1.333-1.755-1.09-.745.083-.73.083-.73 1.205.085 1.84 1.24 1.84 1.24 1.07 1.835 2.805 1.305 3.49.998.108-.775.42-1.305.763-1.605-2.665-.305-5.466-1.335-5.466-5.93 0-1.31.47-2.38 1.235-3.22-.125-.305-.535-1.53.115-3.18 0 0 1.005-.32 3.3 1.23.96-.267 1.98-.4 3-.405 1.02.005 2.04.138 3 .405 2.28-1.55 3.285-1.23 3.285-1.23.655 1.65.245 2.875.12 3.18.77.84 1.235 1.91 1.235 3.22 0 4.61-2.805 5.62-5.475 5.92.43.37.81 1.1.81 2.22 0 1.605-.015 2.895-.015 3.285 0 .32.215.695.825.575C20.565 21.795 24 17.295 24 12c0-6.63-5.37-12-12-12z" />
                    </svg>
                </A>
            </div>
        </div>
    }
}
