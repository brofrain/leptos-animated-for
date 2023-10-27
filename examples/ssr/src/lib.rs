use cfg_if::cfg_if;

pub mod app;
mod button;
pub mod fallback;

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::{view, wasm_bindgen};
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::app::App;

    #[wasm_bindgen]
    pub fn hydrate() {
        // initializes logging using the `log` crate
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        leptos::mount_to_body(move || {
            view! { <App/> }
        });
    }
}}
