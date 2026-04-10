use screeps::{Creep, Part, ResourceType};
use std::any::Any;
use std::fmt::Debug;

pub trait Task: Debug + Any {
    /// Estimate how many ticks are required to fulfill the task for the given creep.
    fn estimate_ticks(&self, creep: &Creep) -> usize;

    /// Execute on tick of work on the given creep
    fn tick(&self, creep: &Creep) -> TaskResult;

    fn get_requirements(&self) -> Vec<TaskRequirement>;
}

#[derive(Debug)]
pub enum TaskRequirement {
    HasUsedCapacity(ResourceType, u32),
    HasFreeCapacity(ResourceType, i32),
    HasParts(Vec<Part>),
}

impl TaskRequirement {
    pub fn does_fulfill(&self, creep: &Creep) -> bool {
        match self {
            TaskRequirement::HasUsedCapacity(resource, amount) =>
                creep.store().get_used_capacity(Some(*resource)) >= *amount,
            TaskRequirement::HasFreeCapacity(resource, amount) =>
                creep.store().get_free_capacity(Some(*resource)) >= *amount,
            TaskRequirement::HasParts(parts) =>
                parts.iter().all(|part| creep.body().iter().any(|body_part| body_part.part() == *part))
        }
    }
}

#[derive(Debug)]
pub enum TaskResult {
    Completed,
    InProgress,
    Error(String),
}