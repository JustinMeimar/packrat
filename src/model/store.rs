use std::sync::{Mutex, OnceLock};
use crate::model::convert::Storable;
use sled::IVec;
use serde_json::Error as SerdeError;

///////////////////////////////////////////////////////////
/// Custom error enum to aggregate error types
#[derive(Debug)]
pub enum StoreError {
    SerdeError(SerdeError),
    SledError(sled::Error),
}

impl From<SerdeError> for StoreError {
    fn from(err: SerdeError) -> Self {
        StoreError::SerdeError(err)
    }
}

impl From<sled::Error> for StoreError {
    fn from(err: sled::Error) -> Self {
        StoreError::SledError(err)
    }
}

///////////////////////////////////////////////////////////

pub struct TaskStore {
    db: Mutex<sled::Db>,
}

impl TaskStore {
    
    /// Create a DB
    pub fn new(db_path: &str) -> Self {
        TaskStore {
            db: sled::open(db_path).unwrap().into()
        }
    }

    /// Singleton instance of TaskManager
    pub fn instance(/* db_path: &str */) -> &'static TaskStore {
        let db_path = "./scratch/patrack.db";

        static INSTANCE: OnceLock<TaskStore> = OnceLock::new();
        
        INSTANCE.get_or_init(|| TaskStore::new(db_path))
    } 
    
    /// 
    pub fn put<T: Storable>(&self, item: T) -> Result<T, StoreError> { 
        let bytes = item.to_bytes()?;
         
        self.db.lock()
            .unwrap()
            .insert(item.to_key(), IVec::from(bytes))?;
        Ok(item)
    }
    
    ///
    pub fn get_prefix<T>(&self, prefix: impl Into<String> + AsRef<[u8]>)
        -> Result<Vec<T>, StoreError> 
    where
        T: Storable,
    
    { 
        let mut results: Vec<T> = self.db
            .lock()
            .unwrap()
            .scan_prefix(prefix)
            .filter_map(|x| x.ok())
            .map(|(_k, v)| T::from_bytes(&v).map_err(StoreError::from))
            .collect::<Result<Vec<T>, StoreError>>()?;

        results.sort_by(|a, b| b.get_timestamp().cmp(&a.get_timestamp()));
        
        Ok(results)
    } 

    /// 
    pub fn get<T: Storable>(&self, key: String) -> Result<Option<T>, StoreError> {
        
        self.db
            .lock()
            .unwrap()
            .get(key)
            .map_err(|e| StoreError::from(e))? // propogate missing get Errors
            .map(|bytes| T::from_bytes(&bytes).map_err(|e| StoreError::from(e)))
            .transpose()
    }
    

    /// 
    pub fn delete_item<T: Storable>(&self, item: &T) -> Result<(), StoreError> {
        
        self.db
            .lock()
            .unwrap()
            .remove(item.to_key())?;
        Ok(()) 
    }
   
    ///
    pub fn delete_key(&self, key: String) -> Result<(), StoreError> {
        self.db
            .lock()
            .unwrap()
            .remove(key)?;
        Ok(()) 
    }
    
    ///
    pub fn truncate(&self) { 
        let db = self.db.lock().unwrap(); 
        db.clear().unwrap();
        db.flush().unwrap();
    }
    
    /// print a debug dump of the store
    pub fn dump(&self) {
        for entry in self.db.lock().unwrap().iter() {
            if let Ok((key, value)) = entry {
                let key_str = String::from_utf8_lossy(&key);
                let value_str = String::from_utf8_lossy(&value);
                println!("Key: {}, Value: {}", key_str, value_str);
            }
        }
    }
}

