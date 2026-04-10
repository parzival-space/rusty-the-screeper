use crate::schedule::helper::{chebyshev_pos, get_creep_move_speed};
use crate::schedule::task::{Task, TaskRequirement, TaskResult};
use log::warn;
use screeps::action_error_codes::HarvestErrorCode;
use screeps::find::SOURCES;
use screeps::Part::{Carry, Move, Work};
use screeps::ResourceType::Energy;
use screeps::{Creep, HasPosition, ResourceType, SharedCreepProperties};

#[derive(Debug)]
pub struct HarvestEnergyTask {}

impl Task for HarvestEnergyTask {
    fn estimate_ticks(&self, creep: &Creep) -> usize {
        let Some(room) = creep.room() else {
            warn!("{} is not in a room, cannot estimate ticks for HarvestEnergyTask", creep.name());
            return usize::MAX;
        };

        let closest_source = room.find(SOURCES, None)
            .into_iter()
            .map(|source| chebyshev_pos(source.pos(), creep.pos()))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(f64::MAX);

        // time needed to move to target
        let move_speed = get_creep_move_speed(creep);
        let travel_ticks = (closest_source / move_speed as f64) as usize;

        // time needed to harvest until full
        let harvest_speed = creep.body().iter().filter(|part| part.part().eq(&Work)).count();
        let capacity_goal = creep.store().get_free_capacity(Some(ResourceType::Energy));
        let harvest_ticks = (capacity_goal as f64 / harvest_speed as f64).ceil() as usize;

        travel_ticks + harvest_ticks
    }

    fn tick(&self, creep: &Creep) -> TaskResult {
        if creep.store().get_free_capacity(Some(Energy)) <= 0 {
            return TaskResult::Completed
        }

        let Some(room) = creep.room() else {
            return TaskResult::Error(
                "Creep is not in a room, cannot execute HarvestEnergyTask".to_string());
        };

        let creep_pos = creep.pos();
        let Some(target) = room.find(SOURCES, None)
            .into_iter()
            .min_by(|a, b| chebyshev_pos(a.pos(), creep_pos)
                .partial_cmp(&chebyshev_pos(b.pos(), creep_pos))
                .unwrap())
        else {
            return TaskResult::Error(
                "No energy sources found in room, cannot execute HarvestEnergyTask".to_string());
        };

        if target.energy() <= 0 {
            return TaskResult::Completed
        }

        let Err(harvest_error): Result<(), HarvestErrorCode> = creep.harvest(&target) else {
            return TaskResult::InProgress;
        };

        match harvest_error {
            HarvestErrorCode::NotInRange => match creep.move_to(&target) {
                Ok(_) => TaskResult::InProgress,
                Err(move_error) => TaskResult::Error(
                    format!("Failed to move to source: {:?}", move_error))
            },
            err => TaskResult::Error(format!("Failed to harvest source: {:?}", err))
        }
    }

    fn get_requirements(&self) -> Vec<TaskRequirement> {
        vec![
            TaskRequirement::HasFreeCapacity(Energy, -1),
            TaskRequirement::HasParts(vec![Move, Work, Carry])
        ]
    }
}

impl HarvestEnergyTask {
    pub fn new() -> Self {
        Self { }
    }
}

