use crate::tasks::actions::{TaskAction, TaskActionError, TaskActionResult};
use crate::tasks::helper::{chebyshev_pos, get_creep_move_speed};
use crate::tasks::requirements::has_body_part_requirement::HasBodyPartRequirement;
use crate::tasks::requirements::TaskRequirement;
use log::trace;
use screeps::action_error_codes::CreepMoveToErrorCode;
use screeps::Part::Move;
use screeps::{Creep, HasPosition, Position, SharedCreepProperties};
use std::any::Any;

#[derive(Debug, Clone)]
pub struct MoveToPositonAction {
    target: Position,
    acceptable_range: usize
}

impl TaskAction for MoveToPositonAction {
    fn tick(&self, creep: &Creep) -> TaskActionResult {
        trace!("Moving to ({}), currently at ({}), distance {}", self.target, creep.pos(), chebyshev_pos(self.target, creep.pos()));
        match creep.move_to(self.target) {
            Ok(_) => {
                if chebyshev_pos(self.target, creep.pos()) as usize <= self.acceptable_range {
                    TaskActionResult::Completed
                } else {
                    TaskActionResult::InProgress
                }
            }
            Err(error) => match error {
                CreepMoveToErrorCode::Tired => TaskActionResult::InProgress,
                error => TaskActionResult::Error(TaskActionError::UnknownError(Box::new(error)))
            }
        }
    }

    fn estimate_ticks(&self, creep: &Creep) -> usize {
        let move_speed = get_creep_move_speed(creep);
        let move_distance = chebyshev_pos(self.target, creep.pos()) as usize;

        (move_distance - self.acceptable_range) / move_speed
    }

    fn get_requirements(&self) -> Vec<Box<dyn TaskRequirement>> {
        vec![
            Box::new(HasBodyPartRequirement::new(Move, 1))
        ]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl MoveToPositonAction {
    pub fn new(target: Position, acceptable_range: usize) -> MoveToPositonAction {
        Self { target, acceptable_range }
    }

    pub fn new_boxed(target: Position, acceptable_range: usize) -> Box<dyn TaskAction> {
        Box::new(Self::new(target, acceptable_range))
    }
}