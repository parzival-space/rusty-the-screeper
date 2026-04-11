use crate::tasks::actions::TaskAction;

pub struct TaskRequest {
    task: Box<dyn TaskAction>,
    priority: usize,
}

impl TaskRequest {
    pub fn new(task: Box<dyn TaskAction>, priority: usize) -> Self {
        Self { task, priority }
    }

    pub fn priority(&self) -> usize {
        self.priority
    }

    pub fn task(&self) -> &dyn TaskAction {
        self.task.as_ref()
    }

    /// Consume the request and return ownership of the underlying task.
    pub fn into_task(self) -> Box<dyn TaskAction> {
        self.task
    }
}