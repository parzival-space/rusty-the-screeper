use crate::extensions::uuid::UuidScreeps;
use crate::tasks::actions::TaskActionResult;
use crate::tasks::resolver::resolve::resolve_task_tree;
use crate::tasks::resolver::task_plan::TaskPlan;
use crate::tasks::task_request::TaskRequest;
use log::{debug, error, info, warn};
use screeps::game::creeps;
use screeps::{Creep, SharedCreepProperties};
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

thread_local! {
    static INSTANCE: RefCell<Option<TaskDispatcher>> = RefCell::new(None);
}

#[derive(Default)]
pub struct TaskDispatcher {
    /// Pending high-level requests todo: sorted by priority
    request_queue: VecDeque<TaskRequest>,
    /// Active plans keyed by a unique plan ID.
    active_plans: HashMap<Uuid, TaskPlan>,
    /// Creep name -> plan ID.
    assignments: HashMap<String, Uuid>
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
        self.request_queue.push_back(request);
    }

    /// Run a scheduler tick.
    pub fn tick(&mut self) {
        self.assign_from_request_queue();
        self.execute_active_plans();
    }


    fn assign_from_request_queue(&mut self) {
        let idle_creeps: Vec<Creep> = creeps()
            .entries()
            .filter(|(name, _)| !self.assignments.contains_key(name))
            .map(|(_, creep)| creep)
            .collect();

        if idle_creeps.is_empty() {
            // no assignment possible, skip tick
            return;
        }

        // try to assign each queued request
        let mut remaining_requests = VecDeque::new();
        for request in self.request_queue.drain(..) {
            let mut assigned = false;

            // find best suitable (creep, plan) pair for this request
            let mut best: Option<(String, TaskPlan)> = None;
            for creep in &idle_creeps {
                let name = creep.name();
                if self.assignments.contains_key(&name) {
                    // already assigned
                    continue;
                }

                let mut plans = resolve_task_tree(creep, request.task());
                if plans.is_empty() {
                    // no route possible for this creep
                    continue;
                }

                // select cheapest plan
                plans.sort_by_key(|p| p.estimated_ticks);
                let cheapest_plan = plans.remove(0);

                match &best {
                    Some((_, existing_plan)) if existing_plan.estimated_ticks <= cheapest_plan.estimated_ticks => {},
                    _ => best = Some((name, cheapest_plan)),
                }
            }

            if let Some((creep_name, plan)) = best {
                let plan_id = Uuid::new_screeps_v4();
                info!(
                    "Assigned plan {} ({} steps, ~{} ticks) to creep {}",
                    plan_id,
                    plan.steps.len(),
                    plan.estimated_ticks,
                    creep_name
                );

                self.active_plans.insert(plan_id, plan);
                self.assignments.insert(creep_name.clone(), plan_id);
                assigned = true;
            }

            if !assigned || request.repeating {
                remaining_requests.push_back(request);
            }
        }
        self.request_queue = remaining_requests;
    }

    fn execute_active_plans(&mut self) {
        for (creep_name, plan_id) in &self.assignments.clone() {
            let Some(creep) = creeps().get(creep_name.clone()) else {
                warn!("Creep {} disappeared. Dropping plan {}.", creep_name, plan_id);
                self.assignments.remove(&creep_name.clone());
                self.active_plans.remove(&plan_id);  // fixme: do not drop the entire plan in this case
                continue;
            };

            let Some(plan) = self.active_plans.get_mut(&plan_id) else {
                error!("Plan {} assigned to creep {} not found! Removing assignment.", plan_id, creep_name);
                self.assignments.remove(&creep_name.clone());
                continue;
            };

            let Some(action) = plan.get_current_task() else {
                // plan completed
                debug!("Plan {} for creep {} completed!", plan_id, creep_name);
                self.assignments.remove(&creep_name.clone());
                self.active_plans.remove(&plan_id);
                continue;
            };

            // execute
            match action.tick(&creep) {
                TaskActionResult::InProgress => { /* still working - do nothing */ }
                TaskActionResult::Completed => {
                    plan.advance_step();
                    debug!(
                        "Creep {} advancing to next step in plan {}, {} steps remaining",
                        creep_name,
                        plan_id,
                        plan.steps.len()
                    );
                }
                TaskActionResult::Error(err) => {
                    error!(
                        "Plan {} step failed for creep {}: {:?}. Aborting plan.",
                        plan_id,
                        creep_name,
                        err
                    );
                    self.assignments.remove(&creep_name.clone());
                    self.active_plans.remove(&plan_id);
                }
            }
        }
    }
}