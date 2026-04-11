use crate::tasks::actions::TaskAction;
use crate::tasks::requirements::TaskRequirement;
use screeps::{Creep, HasStore, ResourceType};
use std::any::Any;

#[derive(Debug)]
pub struct HasFreeCapacityRequirement {
    resource_type: ResourceType,
    free_capacity: i32,
}

impl TaskRequirement for HasFreeCapacityRequirement {
    fn does_meet(&self, creep: &Creep) -> bool {
        creep.store().get_free_capacity(Some(self.resource_type)) >= self.free_capacity
    }

    fn to_meet(&self, _creep: &Creep) -> Option<Vec<Box<dyn TaskAction>>> {
        None // it's not possible to drop filled resources
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl HasFreeCapacityRequirement {
    pub fn new(resource_type: ResourceType, free_capacity: i32) -> Self {
        Self { resource_type, free_capacity }
    }

    pub fn new_boxed(resource_type: ResourceType, free_capacity: i32) -> Box<dyn TaskRequirement> {
        Box::new(Self::new(resource_type, free_capacity))
    }
}