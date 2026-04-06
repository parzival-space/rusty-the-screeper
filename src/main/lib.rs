mod logging;
mod creeps;
pub mod extensions;

use std::cell::RefCell;
use crate::logging::setup_logging;
use log::{debug, info};
use screeps::game::creeps;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::creeps::manager::CreepManager;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(js_name = "init")]
pub fn init() {
    setup_logging();

    debug!("Successfully initialized.");
}

#[wasm_bindgen(js_name = "tick")]
pub fn tick() {
    CreepManager::with(|manager| {
        if creeps().values().count() < 4 {
            manager.spawn_creep(screeps::game::spawns().values().next().unwrap());
        }

        for creep in creeps().values() {
            manager.tick_creep(creep);
        }
    });
}
