use std::fmt;
use sled::Db;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, Local};
use std::fmt::Display;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct TaskEntry {
    pub id: uuid::Uuid,
    pub task_id: uuid::Uuid,
    pub content: Vec<u8>,
    pub timestamp: DateTime<Utc>,
}

impl TaskEntry {
     
    pub fn new(task_id: Uuid, content: Vec<u8>) -> Self {
        TaskEntry {
            id: Uuid::new_v4(),
            task_id,
            content,
            timestamp: Utc::now(), 
        }
    }
}

impl Display for TaskEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.id, self.timestamp, "Preview...") 
    } 
}


