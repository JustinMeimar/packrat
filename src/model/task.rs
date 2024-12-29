use sled::Db;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

struct TaskManager {
    db: sled::Db,
}

impl TaskManager {
    
    /// Create a DB
    pub fn new(db_path: &str) -> Self {
        TaskManager {
            db: sled::open(db_path).unwrap()
        }
    }
    
    /// Create a task in the DB
    pub fn create_task(&self, name: String, desc: String) -> Task {
        let task = Task {
            id: Uuid::new_v4(),
            name,
            desc
        };
        
        task
    }
}

#[derive(Serialize, Deserialize)]
struct Task {
    id: uuid::Uuid,
    name: String,
    desc: String,
}

impl Task {

    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap() 
    }

    pub fn from_bytes(&self, bytes: &[u8]) -> Self {
        serde_json::from_slice(bytes).unwrap()
    }

}

