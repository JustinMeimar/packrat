// task.rs

use std::fmt;
use chrono::{Local, NaiveDate};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use std::fmt::Display;
use crate::model::convert::Storable;

use super::{store::TaskStore, task_entry::TaskEntry}; 

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Task {
    pub id: uuid::Uuid,
    pub name: String,
    pub desc: String,
    pub created_date: NaiveDate,
}

impl Task {
    
    pub fn new<T: Into<String>, K: Into<String>>(name: T, desc: K) -> Self  {
        Task {
            id: Uuid::new_v4(),
            name: name.into(),
            desc: desc.into(),
            created_date: Local::today().naive_local(),
        }
    }
    
    /// get all the entries for a task
    pub fn get_entries(&self) -> Vec<TaskEntry> {
        TaskStore::instance()
            .get_prefix(TaskEntry::key_task(self.id))
            .unwrap()
    }

    /// stateless key pattern for retrieving all task entries
    pub fn key_all() -> &'static str {
        "task:"
    }
    
    /// stateless key pattern for task entries for a specific task
    pub fn key_task<S: Into<String> + Display>(task_id: S) -> String {
        format!("task:{}", task_id)
    } 
}

impl Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "• {}", self.name) 
    } 
}

impl Storable for Task {
    fn to_key(&self) -> String {
        format!("task:{}", self.id)
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

