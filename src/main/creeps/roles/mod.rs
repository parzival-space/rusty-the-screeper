use screeps::{Creep, Part};
use std::fmt::Debug;

mod harvester;
pub use harvester::Harvester;

pub trait CreepRoleHandler: Debug {
    fn new() -> Self
    where
        Self: Sized;
    fn tick(&mut self, creep: &mut Creep);
    fn create_parts_template(available_energy: usize) -> Option<Vec<Part>>
    where
        Self: Sized;
}