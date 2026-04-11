pub mod move_to_positon_action;
pub mod harvest_resource_action;

use crate::tasks::requirements::TaskRequirement;
use screeps::Creep;
use std::any::Any;
use std::error::Error;
use std::fmt::Debug;

pub trait TaskAction: Debug + Any {
    /// Execute a tick for the task.
    fn tick(&self, creep: &Creep) -> TaskActionResult;

    /// Estimate ticks required to complete the task.
    fn estimate_ticks(&self, creep: &Creep) -> usize;

    /// Get a list of requirements to meet for the task.
    fn get_requirements(&self) -> Vec<Box<dyn TaskRequirement>>;

    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub enum TaskActionResult {
    Completed,
    InProgress,
    Error(TaskActionError),
}

#[derive(Debug)]
pub enum TaskActionError {
    UnknownError(Box<dyn Error>),
}