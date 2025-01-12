use std::sync::{Mutex, OnceLock};
use crate::model::convert::Storable;
use sled::IVec;
use serde_json::Error as SerdeError;
use std::error::Error;
use csv::Writer;
use crate::model::task::Task;
use crate::model::task_entry::TaskEntry;

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
    
    /// Idempotent PUT 
    pub fn put<T: Storable>(&self, item: T) -> Result<T, StoreError> { 
        let bytes = item.to_bytes()?;
         
        self.db.lock()
            .unwrap()
            .insert(item.to_key(), IVec::from(bytes))?;
        Ok(item)
    }
    
    /// Key ranged GET for retreiving multiple entities
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

    /// GET a specific value from a key
    pub fn get<T: Storable>(&self, key: String) -> Result<Option<T>, StoreError> {
        
        self.db
            .lock()
            .unwrap()
            .get(key)
            .map_err(|e| StoreError::from(e))? // propogate missing get Errors
            .map(|bytes| T::from_bytes(&bytes).map_err(|e| StoreError::from(e)))
            .transpose()
    }
    

    /// Delete a storable item
    pub fn delete_item<T: Storable>(&self, item: &T) -> Result<(), StoreError> {
        
        /// TODO: Handle cascade!
        self.db
            .lock()
            .unwrap()
            .remove(item.to_key())?;
        Ok(()) 
    }
   
    /// Delete 
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
    
    /// export the DB to a CSV
    pub fn to_csv(&self, file_path: &str) -> Result<(), Box<dyn Error>>{
        
        let mut writer = csv::Writer::from_path(file_path)?;
        let mut columns: Vec<Vec<String>> = vec![]; 
        let tasks: Vec<Task> = self.get_prefix(Task::key_all()).unwrap(); 
    
        for task in &tasks {
            let entries: Vec<String> = task.get_entries()
                .iter()
                .map(|e| e.get_content().replace("\n", ""))
                .collect();
            
            columns.push(entries); 
        }
        
        // determine max number of entries for task, which gives n rows
        let max_len = columns.iter().map(|col| col.len()).max().unwrap_or(0);
        
        // write header row
        let names: Vec<&String> = tasks.iter().map(|t| &t.name).collect();
        writer.write_record(&names);

        // write each row into the CSV
        for i in 0..max_len {
 
            let row: Vec<String> = columns.iter()
                .map(|col| col.get(i).cloned().unwrap_or_else(|| "".to_string()))
                .collect();
            
            writer.write_record(&row);
        }
         
        Ok(()) 
    }
    
    /// export the DB to a JSON file
    pub fn to_json(&self, file_path: &str) -> Result<(), Box<dyn Error>> { 
        panic!("Not implemented error!");
        Ok(()) 
    }
}

