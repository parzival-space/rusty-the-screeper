use crate::extensions::uuid::UuidScreeps;
use crate::tasks::actions::{TaskAction, TaskActionResult};
use crate::tasks::task_request::TaskRequest;
use log::{error, warn};
use screeps::game::creeps;
use screeps::{Creep, SharedCreepProperties};
use std::cell::RefCell;
use std::collections::HashMap;
use uuid::Uuid;

thread_local! {
    static INSTANCE: RefCell<Option<TaskDispatcher>> = RefCell::new(None);
}

#[derive(Default)]
pub struct TaskDispatcher {
    queue: Vec<TaskRequest>,
    active: HashMap<String, Box<dyn TaskAction>>,   // task_id: task
    assignments: HashMap<String, String>,           // creep: task_id
}

impl TaskDispatcher {
    pub fn with<F, R>(f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        INSTANCE.with(|instance| {
            let mut borrow = instance.borrow_mut();
            f(borrow.get_or_insert_with(|| Self::default()))
        })
    }

    /// Submit a new task request to the scheduler.
    pub fn request(&mut self, request: TaskRequest) {
        self.queue.push(request);
    }

    /// Run a scheduler tick.
    pub fn tick(&mut self) {
        let idle_creeps = creeps().entries()
            .filter(|(name, _)| !self.assignments.contains_key(name.as_str()))
            .map(|(_, creep)| creep)
            .collect::<Vec<_>>();

        // iterate over

        // execute active scheduled tasks
        self.assignments.clone().iter().for_each(|(creep_name, task_id)| {
            let Some(creep) = creeps().get(creep_name.clone()) else {
                warn!("Creep {} for Task {} disappeared. Dropping Task.", creep_name, task_id);
                self.assignments.remove(creep_name);
                self.active.remove(task_id);
                return;
            };

            let Some(task) = self.active.get(task_id) else {
                error!("Task {} assigned to creep {} not found in active tasks. This should not happen.", task_id, creep_name);
                self.assignments.remove(creep_name);
                self.active.remove(task_id);
                return;
            };

            match task.tick(&creep) {
                TaskActionResult::Completed => {
                    self.assignments.remove(creep_name);
                    self.active.remove(task_id);
                }
                TaskActionResult::InProgress => {
                    // working, do nothing
                }
                TaskActionResult::Error(error) => {
                    error!("Task {} failed: {:?}", task_id, error);
                    self.active.remove(task_id);
                }
            };
        })
    }
}