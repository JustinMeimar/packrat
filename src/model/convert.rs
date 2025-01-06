// convert.rs

use crate::model::task_entry::TaskEntry;
use std::fmt::Display;

///////////////////////////////////////////////////////////

pub trait Storable: Sized + Display {
    
    // get unique representation
    fn to_key(&self) -> String;

    /// deserialize object from datastore representation
    fn from_bytes(bytes: &[u8]) -> serde_json::Result<Self>;
    
    // serialzie object into bytes for datastore
    fn to_bytes(&self) -> serde_json::Result<Vec<u8>>; 
}

///////////////////////////////////////////////////////////

impl Storable for TaskEntry {
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

