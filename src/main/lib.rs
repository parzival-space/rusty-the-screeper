mod logging;
mod creeps;
pub mod extensions;
pub mod tasks;

use crate::creeps::manager::CreepManager;
use crate::logging::setup_logging;
use crate::tasks::actions::harvest_resource_action::HarvestResourceAction;
use crate::tasks::task_dispatcher::TaskDispatcher;
use crate::tasks::task_request::TaskRequest;
use log::{debug, info};
use screeps::find::RoomObject::Sources;
use screeps::find::SOURCES;
use screeps::game::{creeps, rooms};
use screeps::Source;
use std::cell::RefCell;
use wasm_bindgen::prelude::wasm_bindgen;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(js_name = "init")]
pub fn init() {
    setup_logging();

    debug!("Successfully initialized.");

    let first_room = rooms().values().next().unwrap();
    let first_source_find = first_room.find(SOURCES, None);
    let first_source = first_source_find.get(0).unwrap();
    TaskDispatcher::with(|dispatcher| {
        debug!("Scheduling initial tasks.");
        dispatcher.request(
            TaskRequest::new(
                Box::new(
                    HarvestResourceAction::new(first_source.clone(), 100)), 1)
        );
    });
}

#[wasm_bindgen(js_name = "tick")]
pub fn tick() {
    TaskDispatcher::with(|dispatcher| {
        dispatcher.tick();
    });
}
