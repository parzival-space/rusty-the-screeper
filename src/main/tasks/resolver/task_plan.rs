use crate::tasks::actions::TaskAction;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct TaskPlan {
    pub estimated_ticks: usize,
    /// action queue, first item gets executed first
    pub steps: VecDeque<Box<dyn TaskAction>>,
}

impl TaskPlan {
    pub fn new(estimated_ticks: usize, steps: Vec<Box<dyn TaskAction>>) -> Self {
        Self {
            estimated_ticks,
            steps: steps.into()
        }
    }

    pub fn get_current_task(&self) -> Option<&dyn TaskAction> {
        self.steps.front().map(|t| t.as_ref())
    }

    pub fn advance_step(&mut self) {
        self.steps.pop_front();
    }

    pub fn is_complete(&self) -> bool {
        self.steps.is_empty()
    }
}