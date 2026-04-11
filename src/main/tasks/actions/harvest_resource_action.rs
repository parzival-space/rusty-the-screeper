use crate::tasks::actions::{TaskAction, TaskActionError, TaskActionResult};
use crate::tasks::requirements::has_body_part_requirement::HasBodyPartRequirement;
use crate::tasks::requirements::has_free_capacity_requirement::HasFreeCapacityRequirement;
use crate::tasks::requirements::is_near_position_requirement::IsNearPositionRequirement;
use crate::tasks::requirements::TaskRequirement;
use screeps::Part::{Carry, Work};
use screeps::ResourceType::Energy;
use screeps::{Creep, HasPosition, Mineral, Source};
use std::any::Any;

#[derive(Debug)]
enum HarvestTarget {
    Source(Source),
    Mineral(Mineral),
}

#[derive(Debug)]
pub struct HarvestResourceAction {
    target: HarvestTarget,
    capacity_goal: u32
}

impl TaskAction for HarvestResourceAction {
    fn tick(&self, creep: &Creep) -> TaskActionResult {
        let harvest_result = match &self.target {
            HarvestTarget::Source(source) => creep.harvest(source),
            HarvestTarget::Mineral(mineral) => creep.harvest(mineral)
        };

        match harvest_result {
            Ok(_) => {
                let used_capacity = match &self.target {
                    HarvestTarget::Source(_) => creep.store().get_used_capacity(Some(Energy)),
                    HarvestTarget::Mineral(mineral) => creep.store().get_used_capacity(Some(mineral.mineral_type()))
                };

                if used_capacity >= self.capacity_goal {
                    TaskActionResult::Completed
                } else {
                    TaskActionResult::InProgress
                }
            }
            Err(error) => TaskActionResult::Error(
                TaskActionError::UnknownError(Box::new(error))
            )
        }
    }

    fn estimate_ticks(&self, creep: &Creep) -> usize {
        (self.capacity_goal / creep.get_active_bodyparts(Work) as u32) as usize
    }

    fn get_requirements(&self) -> Vec<Box<dyn TaskRequirement>> {
        let target_pos = match &self.target {
            HarvestTarget::Source(source) => source.pos(),
            HarvestTarget::Mineral(mineral) => mineral.pos()
        };

        vec![
            HasBodyPartRequirement::new_boxed(Work, 1),
            HasBodyPartRequirement::new_boxed(Carry, 1),
            IsNearPositionRequirement::new_boxed(target_pos, 1),
            HasFreeCapacityRequirement::new_boxed(Energy, self.capacity_goal as i32)
        ]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl HarvestResourceAction {
    pub fn new_source(source: Source, capacity_goal: u32) -> Self {
        Self { target: HarvestTarget::Source(source), capacity_goal }
    }

    pub fn new_mineral(mineral: Mineral, capacity_goal: u32) -> Self {
        Self { target: HarvestTarget::Mineral(mineral), capacity_goal }
    }

    pub fn new_boxed_source(source: Source, capacity_goal: u32) -> Box<dyn TaskAction> {
        Box::new(Self::new_source(source, capacity_goal))
    }

    pub fn new_boxed_mineral(mineral: Mineral, capacity_goal: u32) -> Box<dyn TaskAction> {
        Box::new(Self::new_mineral(mineral, capacity_goal))
    }
}