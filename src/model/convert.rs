use crate::model::task::Task;
use crate::model::task_entry::TaskEntry;
use std::fmt::Display;

///////////////////////////////////////////////////////////

pub trait BytesConvertible: Sized {
    fn from_bytes(bytes: &[u8]) -> serde_json::Result<Self>;
    fn to_bytes(&self) -> serde_json::Result<Vec<u8>>;
}

pub trait Storable: Sized + Display {
    
    // get unique representation
    fn to_key(&self) -> String;

    /// deserialize
    fn from_bytes(bytes: &[u8]) -> serde_json::Result<Self>;
    
    // serialzie
    fn to_bytes(&self) -> serde_json::Result<Vec<u8>>;
}

///////////////////////////////////////////////////////////

impl BytesConvertible for TaskEntry {
    
    fn to_bytes(&self) -> serde_json::Result<Vec<u8>> {
        serde_json::to_vec(self) 
    }

    fn from_bytes(bytes: &[u8]) -> serde_json::Result<Self> {
        serde_json::from_slice(bytes)
        // match serde_json::from_slice(bytes) {
        //     Ok(task_entry) => task_entry,
        //     Err(err) => {
        //         panic!(
        //             "Failed to deserialize TaskEntry: {}. Raw data: {:?}",
        //             err, bytes
        //         );
        //     }
        // }
    }

}

impl BytesConvertible for Task {
    
    fn to_bytes(&self) -> serde_json::Result<Vec<u8>> {
        serde_json::to_vec(self)
    }

    fn from_bytes(bytes: &[u8]) -> serde_json::Result<Self> {
        serde_json::from_slice(bytes)
        //     match serde_json::from_slice(bytes) {
        //         Ok(task_entry) => task_entry,
        //         Err(err) => {
        //             panic!(
        //                 "Failed to deserialize Task: {}. Raw data: {:?}",
        //                 err, bytes
        //             );
        //         }
        // }
    }
}

