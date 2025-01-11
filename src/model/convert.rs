// convert.rs

use std::fmt::Display;

///////////////////////////////////////////////////////////

pub trait Storable: Sized + Display {
    
    /// return the fields that are relevant for display
    fn get_display_fields(&self) -> Vec<String>;

    // storable objects have an associated timestamp
    fn get_timestamp(&self) -> String;

    // get unique representation
    fn to_key(&self) -> String;

    /// deserialize object from datastore representation
    fn from_bytes(bytes: &[u8]) -> serde_json::Result<Self>;
    
    // serialzie object into bytes for datastore
    fn to_bytes(&self) -> serde_json::Result<Vec<u8>>;
}

