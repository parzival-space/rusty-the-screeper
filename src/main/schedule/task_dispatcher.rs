use crate::schedule::task::{Task, TaskRequirement, TaskResult};
use crate::schedule::tasks::harvest_energy_task::HarvestEnergyTask;
use crate::schedule::tasks::upgrade_room_controller_task::UpgradeRoomControllerTask;
use log::{debug, error, trace, warn};
use screeps::game::{creeps, rooms};
use screeps::{Creep, HasId, SharedCreepProperties};
use std::any::{type_name_of_val, Any};
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static INSTANCE: RefCell<Option<TaskDispatcher>> = RefCell::new(None);
}

#[derive(Debug)]
pub struct TaskDispatcher {
    pending_tasks: Vec<Box<dyn Task>>,
    assigned_tasks: HashMap<String, Box<dyn Task>>,
}

impl TaskDispatcher {
    pub fn with<F, R>(f: F) -> R
    where
        F: FnOnce(&mut TaskDispatcher) -> R
    {
        INSTANCE.with(|instance| {
            let mut borrow = instance.borrow_mut();
            let dispatcher = borrow.get_or_insert_with(|| {
                TaskDispatcher {
                    pending_tasks: Vec::new(),
                    assigned_tasks: HashMap::new(),
                }
            });

            f(dispatcher)
        })
    }

    fn schedule_tasks(&mut self) {
        for room in rooms().values() {
            // check both already-assigned and still-pending tasks to avoid duplicates
            let already_scheduled = self.assigned_tasks.values()
                .chain(self.pending_tasks.iter())
                .any(|task| (task.as_ref() as &dyn Any).is::<UpgradeRoomControllerTask>());

            if !already_scheduled {
                self.pending_tasks.push(Box::new(UpgradeRoomControllerTask::from_room(&room)));
            }
        }
    }

    fn assign_task(&mut self, task: Box<dyn Task>, creeps: &[Creep]) {
        let best_creep = creeps.iter()
            .filter(|creep| !self.assigned_tasks.contains_key(&creep.name()))
            .filter(|creep| task.get_requirements()
                .iter().all(|req| req.does_fulfill(creep)))
            .min_by_key(|creep| task.estimate_ticks(creep));
        if let Some(creep) = best_creep {
            // a screep can work on this task
            trace!("Assigned task {} to creep {:?}", creep.name(), task.as_ref().type_id());
            self.assigned_tasks.insert(
                creep.name(),
                task
            );
        } else {
            // requirements not fulfilled
            for requirement in task.get_requirements() {
                match requirement {
                    TaskRequirement::HasUsedCapacity(_resource, _) => self.assign_task( // fixme: use resource
                        Box::new(HarvestEnergyTask::new()),
                        creeps,
                    ),

                    // not much we can do about this
                    TaskRequirement::HasFreeCapacity(_, _) => {}
                    TaskRequirement::HasParts(_) => {}
                }
            }
        }
    }

    pub fn evaluate_and_assign(&mut self, creeps: &[Creep]) {
        self.schedule_tasks();

        let tasks: Vec<Box<dyn Task>> = self.pending_tasks.drain(..).collect();
        for task in tasks {
            trace!("TaskDispatcher::evaluate_and_assign");
            self.assign_task(task, creeps);
        }
    }

    pub fn tick_creep(&mut self, creep: &Creep) {
        let name = creep.name();
        let Some(task) = self.assigned_tasks.get_mut(&name) else {
            warn!("TaskDispatcher::tick_creep: creep {} not found", name);
            warn!("Available Task: {:?}", self.assigned_tasks);
            return;
        };

        let result = task.tick(&creep);
        match result {
            TaskResult::InProgress => {}
            TaskResult::Completed => {
                self.assigned_tasks.remove(&name);
            }
            TaskResult::Error(err) => {
                error!("Failed to complete task: {:?}", err);
                self.assigned_tasks.remove(&name);
            }
        }
    }
}