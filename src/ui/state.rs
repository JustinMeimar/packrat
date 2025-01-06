
/// state.rs

use crate::model::task::Task;
use crate::model::task_entry::TaskEntry;
use crate::model::store::TaskStore;
use crate::model::convert::Storable;
use std::time::{Duration, Instant};

///////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct SelectionState {
    pub idx: usize,     // index of current selection
    pub max_idx: usize, // bounds of selection
}

#[derive(Debug)]
pub struct MainViewState {
    pub selector: SelectionState,
    pub items: Vec<Task>,
    pub poll_interval: Duration,
    pub last_poll_time: Instant, 
    // poll_interval = Duration,
    // Duration::from_millis(100);
        // let mut last_poll_time = Instant::now();
}

#[derive(Debug)]
pub struct TaskViewState {
    pub selector: SelectionState,
    pub task: Task,
    pub items: Vec<TaskEntry>,
}

#[derive(Debug)]
pub struct EntryViewState {
    pub task_entry: TaskEntry,
    pub in_editor: bool,
}

#[derive(Debug)]
pub struct CreateViewState<T: Storable> {
    pub item: T,
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
        }
    }
}

impl EntryViewState {
    pub fn new(task: TaskEntry) -> Self {

        EntryViewState {
            task_entry: task,
            in_editor: false
        }
    }
}

impl<T: Storable> CreateViewState<T> {
    
    pub fn new(item: T) -> Self {

        CreateViewState {
            item,
        }
    }
}

