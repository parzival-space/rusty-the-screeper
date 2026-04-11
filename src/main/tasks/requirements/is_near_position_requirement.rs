use crate::tasks::actions::move_to_positon_action::MoveToPositonAction;
use crate::tasks::actions::TaskAction;
use crate::tasks::helper::chebyshev_pos;
use crate::tasks::requirements::TaskRequirement;
use screeps::{Creep, HasPosition, Position};
use std::any::Any;

#[derive(Debug)]
pub struct IsNearPositionRequirement {
    target: Position,
    acceptable_range: usize
}

impl TaskRequirement for IsNearPositionRequirement {
    fn does_meet(&self, creep: &Creep) -> bool {
        let distance = chebyshev_pos(self.target, creep.pos()) as usize;
        distance <= self.acceptable_range
    }

    fn to_meet(&self, _creep: &Creep) -> Option<Vec<Box<dyn TaskAction>>> {
        Some(
            vec![
                MoveToPositonAction::new_boxed(self.target, self.acceptable_range)
            ]
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl IsNearPositionRequirement {
    pub fn new(target: Position, acceptable_range: usize) -> Self {
        Self { target, acceptable_range }
    }

    pub fn new_boxed(target: Position, acceptable_range: usize) -> Box<dyn TaskRequirement> {
        Box::new(Self::new(target, acceptable_range))
    }
}