use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::format;
use log::{debug, error, trace, warn};
use screeps::{game, Creep, ResourceType, SharedCreepProperties, StructureSpawn};
use screeps::action_error_codes::SpawnCreepErrorCode;
use uuid::Uuid;
use crate::creeps::roles::CreepRoleHandler;
use crate::creeps::roles::Harvester;
use crate::extensions::uuid::UuidScreeps;

thread_local! {
    // stores the first created manager instance
    static MANAGER: RefCell<Option<CreepManager>> = RefCell::new(None);
}

#[derive(Debug)]
pub struct CreepManager {
    creep_role_map: HashMap<String, Box<dyn CreepRoleHandler>>
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

    pub fn spawn_creep(&mut self, spawn: StructureSpawn) {
        let creep_name = Uuid::new_screeps_v4().to_string();

        let energy_available = spawn.store().get_used_capacity(Some(ResourceType::Energy)) as usize;
        let Some(creep_template) = Harvester::create_parts_template(energy_available) else {
            warn!("Failed to create creep template for new creep {}. Not spawning.", creep_name);
            return;
        };

        // assign new creep to role
        self.creep_role_map.insert(creep_name.clone(), Box::new(Harvester::new()));

        match spawn.spawn_creep(&creep_template, &creep_name) {
            Ok(_) => {}
            Err(error) => error!("Unexpected error while spawning creep {}: {}", creep_name, error)
        }
    }
}