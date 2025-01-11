use std::fmt;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::fmt::Display;
use crate::{log::debug_log, model::convert::Storable};

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
        write!(f, "• {}\t{}...",
            self.timestamp.format("%Y-%m-%d %H:%M:%S"),
            String::from_utf8_lossy(&self.content)
        ) 
    } 
}

impl Storable for TaskEntry {
 
    /// get a vector of fields that are for display     
    fn get_display_fields(&self) -> Vec<String> { 
        let content = String::from_utf8_lossy(&self.content).to_string() + "...";  
        vec![self.get_timestamp(), content]
    }

    /// datetime object was created
    fn get_timestamp(&self) -> String {
        format!("{}",
            self.timestamp.format("%Y-%m-%d %H:%M:%S"),
        )
    }
    
    /// fill a KV lookup key
    fn to_key(&self) -> String {
        format!("task_entry:{}:{}", self.task_id, self.id)
    }

    /// deserialize object from datastore representation
    fn from_bytes(bytes: &[u8]) -> serde_json::Result<Self> {
        serde_json::from_slice(bytes)
    }
    
    // serialzie object into bytes for datastore
    fn to_bytes(&self) -> serde_json::Result<Vec<u8>> {
        serde_json::to_vec(self) 
    }
}
