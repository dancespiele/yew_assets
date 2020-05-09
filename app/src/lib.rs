#![recursion_limit = "512"]

#[macro_use]
extern crate wasm_bindgen;
extern crate web_sys;
extern crate yew;
extern crate yew_router;
extern crate yew_styles;
use wasm_bindgen::prelude::*;

mod app;
use app::App;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// When the `console_error_panic_hook` feature is enabled, we can call the
// `set_panic_hook` function to get better error messages if we ever panic.
fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() {
    // If the `console_error_panic_hook` feature is enabled this will set a panic hook, otherwise
    // it will do nothing.
    set_panic_hook();

    yew::start_app::<App>();
}
