use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskList {
    pub tasks: Vec<Task>,
}

impl TaskList {
    pub fn new() -> Self {
        TaskList { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, id: u32, description: String) {
        self.tasks.push(Task { id, description, completed: false });
    }

    pub fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks yet!");
        } else {
            for task in &self.tasks {
                let status = if task.completed { "[x]" } else { "[ ]" };
                println!("{} {} - {}", status, task.id, task.description);
            }
        }
    }

    pub fn complete_task(&mut self, id: u32) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            println!("Completed task: {}", id);
        } else {
            println!("Task {} not found", id);
        }
    }

    pub fn delete_task(&mut self, id: u32) {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(pos);
            println!("Deleted task: {}", id);
        } else {
            println!("Task {} not found", id);
        }
    }
}