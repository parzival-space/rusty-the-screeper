use std::cell::RefCell;
use std::collections::HashMap;
use log::{debug, trace, warn};
use screeps::{Creep, SharedCreepProperties};
use crate::creeps::roles::CreepRole;
use crate::creeps::roles::Harvester;

thread_local! {
    // stores the first created manager instance
    static MANAGER: RefCell<Option<CreepManager>> = RefCell::new(None);
}

#[derive(Debug)]
pub struct CreepManager {
    creep_role_map: HashMap<String, Box<dyn CreepRole>>
}

impl CreepManager {
    pub fn with<F, R>(f: F) -> R
    where
        F: FnOnce(&mut CreepManager) -> R,
    {
        // load current manager instance from thread local and persist back into it
        MANAGER.with(|m| {
            let mut borrow = m.borrow_mut();

            let manager = borrow.get_or_insert_with(|| {
                trace!("CreepManager::with_mut creating new manager instance");
                CreepManager {
                    creep_role_map: HashMap::new()
                }
            });

            f(manager)
        })
    }

    pub fn tick_creep(&mut self, mut creep: Creep) {
        let Some(handler) = self.creep_role_map.get_mut(&creep.name()) else {
            debug!("Creep {} has no assigned manager. Assigning to Harvester.", creep.name());
            self.creep_role_map.insert(creep.name().to_string(), Box::new(Harvester::new()));
            return;
        };

        handler.tick(&mut creep);
    }
}