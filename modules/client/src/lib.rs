mod components;
mod models;
mod services;
mod utils;

// export the app component, so it can be used in the main.rs file
pub use components::App;

// loading the locales, to be used in the app
leptos_i18n::load_locales!();
