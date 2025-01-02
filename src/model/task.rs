use std::fmt;
use sled::Db;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, Local};
use std::fmt::Display;
use crate::model::convert::BytesConvertible; 

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Task {
    pub id: uuid::Uuid,
    pub name: String,
    pub desc: String,
}

impl Task {
    
    pub fn new(name: String, desc: String) -> Self {
        Task { id: Uuid::new_v4(), name, desc, }
    } 
}

impl Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.id, self.name, self.desc) 
    } 
}

