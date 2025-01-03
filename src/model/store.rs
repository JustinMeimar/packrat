use std::fmt;
use sled::Db;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, Local};
use std::sync::{Mutex, LazyLock};
use std::fmt::Display;
use std::sync::{Mutex, OnceLock};
use crate::model::task::Task;
use crate::model::task_entry::TaskEntry;
use crate::model::convert::Storable;

pub struct TaskStore {
    db: Mutex<sled::Db>,
}

impl TaskStore {
    
    /// Create a DB
    pub fn new(db_path: &str) -> Self {
        TaskStore {
            db: sled::open(db_path).unwrap()
        }
    }

    /// Singleton instance of TaskManager
    pub fn instance(db_path: &str) -> &'static TaskStore {
        static INSTANCE: OnceLock<TaskStore> = OnceLock::new();
        
        INSTANCE.get_or_init(|| TaskStore::new(db_path))
    } 
    
    /// 
    pub fn put<T: Storable>(&self, item: T) -> sled::Result<T> { 
        self.db.lock()
            .unwrap()
            .insert(
                item.to_key(),
                item.to_bytes()?,
            )?;
        Ok(item)
    }
    
    ///
    pub fn get_prefix<T: Storable>(&self, prefix: String) -> sled::Result<Vec<T>> {
        self.db
            .lock()
            .unwrap()
            .scan_prefix(prefix)
            .filter_map(|x| x.ok()) // only take some values
            .map(|(_k, v)| T::from_bytes(&v).unwrap())
            .collect()
    }

    /// 
    pub fn get<T: Storable>(&self, key: String) -> sled::Result<Option<T>> {
        self.db
            .lock()
            .unwrap()
            .get(key)
            .ok()
            .flatten()
            .map(|data| T::from_bytes(&data).unwrap()?)
    }

    /// 
    pub fn delete_item<T: Storable>(&self, item: T) -> sled::Result<()> {
        self.db
            .lock()
            .unwrap()
            .remove(item.to_key())?;
        Ok(()) 
    }
   
    ///
    pub fn delete_key(&self, key: String) -> sled::Result<()> {
        self.db
            .lock()
            .unwrap()
            .remove(key)?;
        Ok(()) 
    }
}

