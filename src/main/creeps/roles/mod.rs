use std::fmt::Debug;
use screeps::Creep;

mod harvester;
pub use harvester::Harvester;

pub trait CreepRole: Debug {

    fn new() -> Self
    where
        Self: Sized;
    fn tick(&mut self, creep: &mut Creep);
}