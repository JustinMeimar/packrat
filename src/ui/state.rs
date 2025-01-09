
/// state.rs

use crate::model::task::Task;
use crate::model::task_entry::TaskEntry;
use crate::model::store::TaskStore;
use crate::model::convert::Storable;
use std::time::{Duration, Instant};

///////////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub struct SelectionState {
    pub idx: usize,     // index of current selection
    pub max_idx: usize, // bounds of selection
}

#[derive(Debug, PartialEq)]
pub struct MainViewState {
    pub selector: SelectionState,
    pub items: Vec<Task>,
    pub poll_interval: Duration,
    pub last_poll_time: Instant,
}

#[derive(Debug, PartialEq)]
pub struct TaskViewState {
    pub selector: SelectionState,
    pub task: Task,
    pub items: Vec<TaskEntry>,
    pub poll_interval: Duration,
    pub last_poll_time: Instant,
}

#[derive(Debug, PartialEq)]
pub struct EntryViewState {
    pub task_entry: TaskEntry,
    pub in_editor: bool,
}

#[derive(Debug, PartialEq)]
pub struct CreateViewState<T: Storable> {
    pub item: T,
    pub inputs: Vec<String>, 
    pub active_input: usize,
}

///////////////////////////////////////////////////////////

impl SelectionState {
    
    pub fn new(max_idx: usize) -> Self {
        SelectionState {idx: 0, max_idx}
    }
    
    pub fn expand(&mut self) { self.max_idx += 1; }
    
    /// clamp between 0 and n 
    pub fn shrink(&mut self) {
        self.max_idx = std::cmp::min(self.max_idx-1, 0);
        self.idx = std::cmp::min(self.idx, 0)
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

impl MainViewState {

    pub fn new() -> Self {

        let tasks = TaskStore::instance()
            .get_prefix(Task::key_all())
            .unwrap();
    
        MainViewState {
            selector: SelectionState::new(tasks.len()),
            items: tasks,
            poll_interval: Duration::from_millis(100),
            last_poll_time: Instant::now(),
        }
    }    
}

impl TaskViewState {
    pub fn new(task: Task) -> Self {

        let task_entries = TaskStore::instance()
            .get_prefix(TaskEntry::key_task(task.id))
            .unwrap();
        
        assert!(task_entries.len() >= 0);

        TaskViewState {
            selector: SelectionState::new(task_entries.len()),
            items: task_entries,
            task,
            poll_interval: Duration::from_millis(100),
            last_poll_time: Instant::now(),
        }
    }
}

// View for task entries
impl EntryViewState {
    pub fn new(task: TaskEntry) -> Self {

        EntryViewState {
            task_entry: task,
            in_editor: false
        }
    }
}

impl CreateViewState<Task> {
    pub fn new(item: Task) -> Self {
        CreateViewState {
            item,
            // TODO: Find a way to encapsulate this within a Task!
            // Perhaps get_default_inputs()
            inputs: vec![
                String::from("Task Title"),
                String::from("Task Description")
            ],
            active_input: 0
        }
    }
}
