use crate::tasks::actions::harvest_resource_action::HarvestResourceAction;
use crate::tasks::actions::TaskAction;
use crate::tasks::helper::chebyshev_pos;
use crate::tasks::requirements::TaskRequirement;
use screeps::find::{MINERALS, SOURCES};
use screeps::{Creep, HasPosition, ResourceType};
use std::any::Any;

#[derive(Debug)]
pub struct HasUsedCapacityRequirement {
    resource_type: ResourceType,
    capacity: u32,
}

impl TaskRequirement for HasUsedCapacityRequirement {
    fn does_meet(&self, creep: &Creep) -> bool{
        creep.store().get_used_capacity(Some(self.resource_type)) >= self.capacity
    }

    fn to_meet(&self, creep: &Creep) ->  Option<Vec<Box<dyn TaskAction>>> {
        let Some(room) = creep.room() else {
            return None;
        };
        
        match self.resource_type { 
            ResourceType::Energy => room.find(SOURCES, None).iter()
                .min_by_key(|instance| chebyshev_pos(creep.pos(), instance.pos()) as usize)
                .map(|instance| HarvestResourceAction::new_boxed_source(instance.clone(), self.capacity))
                .into_iter().next().map(|i| vec![i]),
            _ => room.find(MINERALS, None).iter()
                .min_by_key(|instance| chebyshev_pos(creep.pos(), instance.pos()) as usize)
                .map(|instance| HarvestResourceAction::new_boxed_mineral(instance.clone(), self.capacity))
                .into_iter().next().map(|i| vec![i]),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl HasUsedCapacityRequirement {
    pub fn new(resource_type: ResourceType, capacity: u32) -> Self {
        Self { resource_type, capacity }
    }

    pub fn new_boxed(resource_type: ResourceType, capacity: u32) -> Box<dyn TaskRequirement> {
        Box::new(Self::new(resource_type, capacity))
    }
}