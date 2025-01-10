// convert.rs

use std::fmt::Display;

///////////////////////////////////////////////////////////

pub trait Storable: Sized + Display {
    
    // fn new() -> Self;
    
    // get unique representation
    fn to_key(&self) -> String;

    /// deserialize object from datastore representation
    fn from_bytes(bytes: &[u8]) -> serde_json::Result<Self>;
    
    // serialzie object into bytes for datastore
    fn to_bytes(&self) -> serde_json::Result<Vec<u8>>;
}

