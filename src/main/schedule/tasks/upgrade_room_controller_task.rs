use crate::schedule::helper::{chebyshev, get_creep_move_speed};
use crate::schedule::task::{Task, TaskRequirement, TaskResult};
use screeps::action_error_codes::UpgradeControllerErrorCode;
use screeps::Part::{Carry, Move, Work};
use screeps::ResourceType::Energy;
use screeps::{Creep, HasId, HasPosition, ObjectId, Room, StructureController};

#[derive(Debug)]
pub struct UpgradeRoomControllerTask {
    target: ObjectId<StructureController>,
}

impl Task for UpgradeRoomControllerTask {
    fn estimate_ticks(&self, creep: &Creep) -> usize {
        let Some(target) = self.target.resolve() else {
            return usize::MAX;
        };

        let move_speed = get_creep_move_speed(creep);
        let distance = chebyshev(
            vec![target.pos().x().u8() as f64, target.pos().y().u8() as f64].as_slice(),
            vec![creep.pos().x().u8() as f64, creep.pos().y().u8() as f64].as_slice()
        );

        (distance / move_speed as f64) as usize
    }

    fn tick(&self, creep: &Creep) -> TaskResult {
        let Some(target) = self.target.resolve() else {
            return TaskResult::Error(
                format!("Room controller target {:?} not found", creep.pos()),
            )
        };

        if target.level() >= 8
            && let Some(downgrade_timer) = target.ticks_to_downgrade()
            && downgrade_timer >= 150000 {
            return TaskResult::Completed
        }

        let Err(upgrade_error) = creep.upgrade_controller(target.as_ref()) else {
            return TaskResult::InProgress;
        };

        match upgrade_error {
            UpgradeControllerErrorCode::NotInRange => match creep.move_to::<StructureController>(target) {
                Ok(_) => TaskResult::InProgress,
                Err(move_error) => TaskResult::Error(
                    format!("Failed to move to controller: {:?}", move_error))
            },
            err => TaskResult::Error(format!("Failed to upgrade controller: {:?}", err))
        }
    }

    fn get_requirements(&self) -> Vec<TaskRequirement> {
        let Some(target) = self.target.resolve() else {
            return vec![]; // target not found, task cannot be completed
        };
        vec![
            TaskRequirement::HasParts(vec![Move, Carry, Work]),
            TaskRequirement::HasUsedCapacity(Energy, 1) // has energy to upgrade
        ]
    }
}

impl UpgradeRoomControllerTask {
    pub fn from_room(room: &Room) -> Self {
        let Some(controller) = room.controller() else {
            panic!("Room has no controller.");
        };

        Self {
            target: controller.id(),
        }
    }
}