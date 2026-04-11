use crate::tasks::actions::TaskAction;
use screeps::Creep;


struct TaskPlan {
    pub cost: usize,
    pub tasks: Vec<Box<dyn TaskAction>>,
}

pub fn resolve_task_tree(creep: Creep, task: Box<dyn TaskAction>) -> Vec<TaskPlan> {
    let base_plan = TaskPlan {
        cost: task.estimate_ticks(&creep),
        tasks: vec![],
    };

    let requirements = task.get_requirements();
    let mut plans: Vec<TaskPlan> = vec![base_plan];

    for req in requirements.iter() {
        if req.does_meet(&creep) {
            continue;
        }

        let Some(prereqs) = req.to_meet(&creep) else {
            return vec![];
        };

        let mut new_plans: Vec<TaskPlan> = Vec::new();

        for plan in &plans {
            for prereq in &prereqs {
                let prereq_plans = resolve_task_tree(creep.clone(), Box::new(prereq.clone()));
                if prereq_plans.is_empty() {
                    continue;
                }

                for prereq_plan in prereq_plans {
                    new_plans.push(TaskPlan {
                        cost: plan.cost + prereq_plan.cost,
                        tasks: [prereq_plan.tasks.clone(), plan.tasks.clone()].concat(),
                    });
                }
            }
        }

        plans = new_plans;
    }

    // append the task itself to all plans
    for plan in &mut plans {
        plan.tasks.push(task.clone());
        plan.cost += task.estimate_ticks(&creep);
    }

    plans
}