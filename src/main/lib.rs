mod logging;
mod creeps;
pub mod extensions;
pub mod tasks;

use crate::logging::setup_logging;
use crate::tasks::actions::upgrade_room_controller_action::UpgradeRoomControllerAction;
use crate::tasks::task_dispatcher::TaskDispatcher;
use crate::tasks::task_request::TaskRequest;
use screeps::game::rooms;
use wasm_bindgen::prelude::wasm_bindgen;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(js_name = "init")]
pub fn init() {
    setup_logging();
    TaskDispatcher::with(|dispatcher| {
        for room in rooms().values() {
            dispatcher.request(
                TaskRequest::repeating(
                    Box::new(UpgradeRoomControllerAction::new_room(room)),
                    1
                )
            );
        }
    });
}

#[wasm_bindgen(js_name = "tick")]
pub fn tick() {
    TaskDispatcher::with(|dispatcher| {
        dispatcher.tick();
    });
}
