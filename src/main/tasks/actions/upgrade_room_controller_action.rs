use crate::tasks::actions::{TaskAction, TaskActionError, TaskActionResult};
use crate::tasks::requirements::has_body_part_requirement::HasBodyPartRequirement;
use crate::tasks::requirements::has_used_capacity_requirement::HasUsedCapacityRequirement;
use crate::tasks::requirements::is_near_position_requirement::IsNearPositionRequirement;
use crate::tasks::requirements::TaskRequirement;
use screeps::action_error_codes::UpgradeControllerErrorCode;
use screeps::Part::{Carry, Work};
use screeps::ResourceType::Energy;
use screeps::{Creep, HasPosition, Room, StructureController};
use std::any::Any;

// level 8 controller timer is 200000 ticks. the upgrade goal is 8, so filling the level 7 timer
// should ensure we reach level 8
const CONTROLLER_LEVEL_7_TIMER_GOAL: u32 = 150000;
// see https://docs.screeps.com/api/#StructureController
const CONTROLLER_LEVEL_8_ENERGY_GOAL: u32 = 16380200;

#[derive(Clone, Debug)]
pub struct UpgradeRoomControllerAction {
    target: StructureController,
}

impl TaskAction for UpgradeRoomControllerAction {
    fn tick(&self, creep: &Creep) -> TaskActionResult {
        match creep.upgrade_controller(&self.target) {
            Ok(_) => {
                if self.target.level() >= 8 && self.target.ticks_to_downgrade() >= Some(CONTROLLER_LEVEL_7_TIMER_GOAL) {
                    TaskActionResult::Completed
                } else {
                    TaskActionResult::InProgress
                }
            }
            Err(error) => match error {
                UpgradeControllerErrorCode::NotEnoughResources => TaskActionResult::Completed,
                error => TaskActionResult::Error(TaskActionError::UnknownError(Box::new(error)))
            }
        }
    }

    fn estimate_ticks(&self, creep: &Creep) -> usize {
        let creep_transfer_speed = creep.get_active_bodyparts(Work);
        // todo: include downgrade timer in estimation
        CONTROLLER_LEVEL_8_ENERGY_GOAL as usize / creep_transfer_speed as usize
    }

    fn get_requirements(&self) -> Vec<Box<dyn TaskRequirement>> {
        vec![
            HasBodyPartRequirement::new_boxed(Work, 1),
            HasBodyPartRequirement::new_boxed(Carry, 1),
            IsNearPositionRequirement::new_boxed(self.target.pos(), 3),
            HasUsedCapacityRequirement::new_boxed(Energy, 1)
        ]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl UpgradeRoomControllerAction {
    pub fn new_room(room: Room) -> Self {
        let Some(controller) = room.controller() else {
            panic!("Room {} does not have a controller", room.name());
        };
        Self { target: controller }
    }
}