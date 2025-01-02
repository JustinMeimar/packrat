use crate::model::task::Task;
use crate::model::task_entry::TaskEntry;

///////////////////////////////////////////////////////////

pub trait BytesConvertible: Sized {
    fn from_bytes(bytes: &[u8]) -> Self;
    fn to_bytes(&self) -> Vec<u8>;
}

///////////////////////////////////////////////////////////

impl BytesConvertible for TaskEntry {
    
    fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap() 
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        match serde_json::from_slice(bytes) {
            Ok(task_entry) => task_entry,
            Err(err) => {
                panic!(
                    "Failed to deserialize TaskEntry: {}. Raw data: {:?}",
                    err, bytes
                );
            }
        }
    }

}

impl BytesConvertible for Task {
    
    fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap() 
    }

    fn from_bytes(bytes: &[u8]) -> Self {
            match serde_json::from_slice(bytes) {
                Ok(task_entry) => task_entry,
                Err(err) => {
                    panic!(
                        "Failed to deserialize Task: {}. Raw data: {:?}",
                        err, bytes
                    );
                }
        }
    }
}

