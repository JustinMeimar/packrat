use crate::model::task::{TaskManager, Task, TaskEntry};

///////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct SelectionState {
    pub idx: usize,     // index of current selection
    pub max_idx: usize, // bounds of selection
}

impl SelectionState {
    
    pub fn new(max_idx: usize) -> Self {
        SelectionState {idx: 0, max_idx}
    }
    
    pub fn incr(&mut self) {
        if self.max_idx != 0 {
            self.idx = (self.idx + self.max_idx - 1) % self.max_idx;
        } 
    }

    pub fn decr(&mut self) {
        if self.max_idx != 0 {
            self.idx = (self.idx + self.max_idx + 1) % self.max_idx;
        }
    }
}

#[derive(Debug)]
pub struct MainViewState {
    pub selector: SelectionState,
    pub items: Vec<Task>,
}

impl MainViewState {

    pub fn new() -> Self {
        let tasks = TaskManager::instance()
                .lock()
                .unwrap()
                .get_tasks();

        MainViewState {
            selector: SelectionState::new(tasks.len()),
            items: tasks
        }
    }

}

#[derive(Debug)]
pub struct TaskViewState {
    pub selector: SelectionState,
    pub task: Task,
    pub items: Vec<TaskEntry>,
}

impl TaskViewState {
    pub fn new(task: Task) -> Self {

        let task_entries = TaskManager::instance()
                .lock()
                .unwrap()
                .get_task_entries(task.id);

        TaskViewState {
            selector: SelectionState::new(task_entries.len()),
            items: task_entries,
            task,
        }
    }
}

#[derive(Debug)]
pub struct EntryViewState {
    pub task_entry: TaskEntry,
    pub in_editor: bool,
}

impl EntryViewState {
    pub fn new(task: TaskEntry) -> Self {

        EntryViewState {
            task_entry: task,
            in_editor: false
        }
    }
}

