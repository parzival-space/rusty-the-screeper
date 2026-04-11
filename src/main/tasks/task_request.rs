use crate::tasks::actions::TaskAction;

pub struct TaskRequest {
    task: Box<dyn TaskAction>,
    priority: usize,
    pub repeating: bool,
}

impl TaskRequest {
    pub fn new(task: Box<dyn TaskAction>, priority: usize) -> Self {
        Self { task, priority, repeating: false }
    }

    pub fn repeating(task: Box<dyn TaskAction>, priority: usize) -> Self {
        Self { task, priority, repeating: true }
    }

    pub fn priority(&self) -> usize {
        self.priority
    }

    pub fn task(&self) -> Box<dyn TaskAction> {
        self.task.clone()
    }

    /// Consume the request and return ownership of the underlying task.
    pub fn into_task(self) -> Box<dyn TaskAction> {
        self.task
    }
}