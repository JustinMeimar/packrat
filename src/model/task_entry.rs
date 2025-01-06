use std::fmt;
use sled::Db;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, Local};
use std::fmt::Display;
use chrono::format::format;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct TaskEntry {
    pub id: uuid::Uuid,
    pub task_id: uuid::Uuid,
    pub content: Vec<u8>,
    pub timestamp: DateTime<Utc>,
}

impl TaskEntry {
     
    pub fn new<T: Into<Vec<u8>>>(task_id: Uuid, content: T) -> Self {
        TaskEntry {
            id: Uuid::new_v4(),
            task_id,
            content: content.into(),
            timestamp: Utc::now(), 
        }
    }
    
    /// stateless key pattern for retrieving all task entries
    pub fn key_all() -> &'static str {
        "task_entry:"
    }
    
    /// stateless key pattern for task entries for a specific task
    pub fn key_task<S: Into<String> + Display>(task_id: S) -> String {
        format!("task_entry:{}", task_id)
    }
    
    /// stateless key pattern for specific entry for a specific task
    pub fn key_task_entry<S: Into<String> + Display>(task_id: S, entry_id: S) -> String {
        format!("task_entry:{}:{}", task_id, entry_id)
    }
}

impl Display for TaskEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.id, self.timestamp, "Preview...") 
    } 
}


