use crate::creeps::roles::CreepRoleHandler;
use log::{trace, warn};
use screeps::action_error_codes::{HarvestErrorCode, TransferErrorCode, UpgradeControllerErrorCode};
use screeps::Part::{Carry, Move, Work};
use screeps::{find, BodyPart, Creep, Harvestable, Part, ResourceType, RoomObjectProperties, SharedCreepProperties, StructureController, Transferable};
use std::fmt::{Debug, Formatter};
use std::mem;
use std::sync::Arc;

static HARVESTER_TEMPLATE_MIN: &'static [Part] = &[Carry, Move, Work];
static HARVESTER_TEMPLATE_SCALE: &'static [Part] = &[Carry, Move];

#[derive(Debug)]
pub struct Harvester {
    state: HarvesterState,
}

impl CreepRoleHandler for Harvester {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            state: HarvesterState::Idle,
        }
    }

    fn tick(&mut self, creep: &mut Creep) {
        match &mut self.state.clone() {
            HarvesterState::Idle => self.idle(creep),
            HarvesterState::Harvesting(target, resource) => self.harvest(creep, &target, &resource),
            HarvesterState::UpgradingController(target) => self.upgrade_controller(creep, &target),
            HarvesterState::Transferring(target, resource) => self.transfer_resource(creep, &target, &resource)
        }
    }

    fn create_parts_template(available_energy: usize) -> Option<Vec<Part>> {
        let mut parts = HARVESTER_TEMPLATE_MIN.to_vec();
        let mut current_cost: usize = parts.iter().map(|p| p.cost() as usize).sum();

        if current_cost > available_energy {
            return None;
        }

        let mut scale_iter = HARVESTER_TEMPLATE_SCALE.iter().cycle();

        while let Some(part) = scale_iter.next() {
            let part_cost = part.cost() as usize;
            if current_cost + part_cost > available_energy {
                break;
            }
            parts.push(*part);
            current_cost += part_cost;
        }

        Some(parts)
    }
}

impl Harvester {
    fn next_state(&mut self, new_state: HarvesterState) {
        trace!("Transitioning from state {:?} to state {:?}...", self.state, new_state);
        self.state = new_state;
    }

    fn idle(&mut self, creep: &mut Creep) {
        if creep.store().get_free_capacity(Some(ResourceType::Energy)) > 0 {
            // find energy source and enter harvesting state
            let Some(room) = creep.room() else {
                warn!("Creep {} is not in a room.", creep.name());
                return;
            };
            let find = room.find(find::SOURCES, None);
            let Some(source) = find.get(0) else {
                warn!("Creep {} is not in a room containing a source.", creep.name());
                return;
            };
            self.next_state(
                HarvesterState::Harvesting(Arc::new(source.clone()), ResourceType::Energy)
            );
        } else {
            // find controller and enter upgrading state
            let Some(room) = creep.room() else {
                warn!("Creep {} is not in a room.", creep.name());
                return;
            };
            let Some(controller) = room.controller() else {
                warn!("Creep {} is in a room without a controller.", creep.name());
                return;
            };
            self.next_state(
                HarvesterState::UpgradingController(controller)
            );
        }
    }

    fn harvest(&mut self, creep: &mut Creep, target: &Arc<dyn Harvestable>, resource: &ResourceType) {
        if creep.store().get_free_capacity(Some(*resource)) > 0 {
            match creep.harvest(target.as_ref()) {
                Ok(_) => { /* do something */ }
                Err(harvest_error) => match harvest_error {
                    HarvestErrorCode::NotInRange => creep.move_to(target.as_ref()).unwrap_or(()),
                    _ => warn!("Failed to harvest energy. Creep: {}, Error: {}", creep.name(), harvest_error)
                }
            }
        } else {
            // enter next state
            self.state = HarvesterState::Idle;
        }
    }

    fn upgrade_controller(&mut self, creep: &mut Creep, target: &StructureController) {
        if creep.store().get_used_capacity(Some(ResourceType::Energy)) > 0 {
            match creep.upgrade_controller(&target) {
                Ok(_) => { /* do something */ }
                Err(upgrade_error) => match upgrade_error {
                    UpgradeControllerErrorCode::NotInRange => creep.move_to::<StructureController>(target.clone()).unwrap_or(()),
                    _ => warn!("Failed to transfer energy to controller. Creep: {}, Error: {}", creep.name(), upgrade_error)
                }
            }
        } else {
            // enter next state
            trace!("Creep {} has no energy left. Transitioning back to Idle state.", creep.name());
            self.state = HarvesterState::Idle;
        }
    }

    fn transfer_resource(&mut self, creep: &mut Creep, target: &Arc<dyn Transferable>, resource: &ResourceType) {
        if creep.store().get_free_capacity(Some(*resource)) > 0 {
            match creep.transfer(target.as_ref(), *resource, None) {
                Ok(_) => { /* do something */ }
                Err(transfer_error) => match transfer_error {
                    TransferErrorCode::NotInRange => creep.move_to(target.as_ref()).unwrap_or(()),
                    _ => warn!("Failed to transfer energy to spawn. Creep: {}, Error: {}", creep.name(), transfer_error)
                }
            }
        } else {
            // enter next state
            self.state = HarvesterState::Idle;
        }
    }
}

#[derive(Clone)]
enum HarvesterState {
    Idle,
    Harvesting(Arc<dyn Harvestable>, ResourceType),
    UpgradingController(StructureController),
    Transferring(Arc<dyn Transferable>, ResourceType),
}

impl Debug for HarvesterState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HarvesterState::Idle => write!(f, "Idle"),
            HarvesterState::Harvesting(target, resource) => write!(f, "Harvesting(target: {:?}, resource: {})", target.as_ref().as_ref(), resource),
            HarvesterState::UpgradingController(target) => write!(f, "UpgradingController(target: {:?})", target),
            HarvesterState::Transferring(target, resource) => write!(f, "Transferring(target: {:?}, resource: {})", target.as_ref().as_ref(), resource)
        }
    }
}