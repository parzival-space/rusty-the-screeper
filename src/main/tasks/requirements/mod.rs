pub mod has_used_capacity_requirement;
pub mod has_free_capacity_requirement;
pub mod has_body_part_requirement;
pub mod is_near_position_requirement;

use crate::tasks::actions::TaskAction;
use screeps::Creep;
use std::any::Any;
use std::fmt::Debug;

pub trait TaskRequirement: Debug + Any {
    /// Whether the creep meets the requirement or not.
    fn does_meet(&self, creep: &Creep) -> bool;

    /// Tasks required to be executed to meet the requirements.
    fn to_meet(&self, creep: &Creep) -> Option<Vec<Box<dyn TaskAction>>>;

    fn as_any(&self) -> &dyn Any;
}