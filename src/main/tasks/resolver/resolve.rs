use crate::tasks::actions::TaskAction;
use crate::tasks::resolver::task_plan::TaskPlan;
use screeps::Creep;

pub fn resolve_task_tree(creep: &Creep, task: Box<dyn TaskAction>) -> Vec<TaskPlan> {
    let requirements = task.get_requirements();
    let base_cost = task.estimate_ticks(creep);

    // start with an empty "seed" plan
    let mut plans: Vec<(usize, Vec<Box<dyn TaskAction>>)> = vec![(0, vec![])];

    for requirement in requirements {
        if requirement.does_meet(creep) {
            // skip if already meet
            continue;
        }

        let Some(requirement_actions) = requirement.to_meet(creep) else {
            // unsatisfiable -> This Task is impossible
            return vec![];
        };

        let mut new_plans = Vec::new();
        for (plan_cost, plan_steps) in &plans {
            for requirement_action in &requirement_actions {
                // recursively resolve the prerequisite itself
                let sub_plans = resolve_task_tree(creep, requirement_action.clone());
                if sub_plans.is_empty() {
                    continue;
                }

                for sub_plan in sub_plans {
                    let mut combined_steps = plan_steps.clone();
                    combined_steps.extend(sub_plan.steps);
                    new_plans.push((
                        plan_cost + sub_plan.estimated_ticks,
                        combined_steps
                    ));
                }
            }
        }

        if new_plans.is_empty() {
            return vec![]; // no viable path
        }
        plans = new_plans;
    }

    // append the original task as the final step of every plan
    plans.into_iter()
        .map(|(cost, mut steps)| {
            steps.push(task.clone());
            TaskPlan::new(cost + base_cost, steps)
        })
        .collect()
}