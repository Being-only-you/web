pub mod app;
pub mod components;
pub mod content;
pub mod pages;
pub mod error;
pub mod db;
pub mod config;
pub mod utils;
pub mod actix_session_surrealdb;
#[macro_use]
pub mod prelude;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}