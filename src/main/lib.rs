use log::info;
use wasm_bindgen::prelude::wasm_bindgen;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(js_name = "init")]
pub fn init() {
    info!("Initializing JS");
}

#[wasm_bindgen(js_name = "tick")]
pub fn tick() {
    info!("Ticking");
}