use crate::tasks::actions::TaskAction;
use crate::tasks::requirements::TaskRequirement;
use screeps::{Creep, Part};
use std::any::Any;

#[derive(Debug)]
pub struct HasBodyPartRequirement {
    part: Part,
    min_required: u8,
}

impl TaskRequirement for HasBodyPartRequirement {
    fn does_meet(&self, creep: &Creep) -> bool {
        creep.get_active_bodyparts(self.part) >= self.min_required
    }

    fn to_meet(&self, _creep: &Creep) -> Option<Vec<Box<dyn TaskAction>>> {
        None // creeps cannot update their body parts
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl HasBodyPartRequirement {
    pub fn new(part: Part, min_required: u8) -> Self {
        Self { part, min_required }
    }

    pub fn new_boxed(part: Part, min_required: u8) -> Box<dyn TaskRequirement> {
        Box::new(Self::new(part, min_required))
    }
}