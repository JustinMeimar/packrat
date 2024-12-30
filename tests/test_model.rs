use packrat::model::task::{Task, TaskManager, BytesConvertible};
use tempfile::TempDir;

#[test]
fn test_task() {
    
    // create task
    let task = Task::new(String::from("clean"), String::from("do the dishes"));
    assert_eq!(task.name, "clean");
    assert_eq!(task.desc, "do the dishes");
        
    // serialize and deserialize
    let ser_task = task.to_bytes();
    let de_task = Task::from_bytes(&ser_task);
    assert_eq!(de_task, task);
}

#[test]
fn test_task_manager() {
   
    let db_path = TempDir::new().unwrap().path().join("test.db");
    let tm = TaskManager::new(db_path.to_str().unwrap());
    tm.truncate();

    assert_eq!(tm.get_tasks().len(), 0);

    let task1 = tm.create_task(
        String::from("run"),
        String::from("run around the block")
    );
     
    let task2 = tm.create_task(
        String::from("soccer"),
        String::from("play some footy mate")
    );
     
    assert_eq!(tm.get_task(task2.id).unwrap(), task2); 
    assert_eq!(tm.get_task(task1.id).unwrap(), task1);    
    assert_eq!(tm.get_tasks().len(), 2);
}

#[test]
fn test_task_entry_manager() {
  

    let db_path = TempDir::new().unwrap().path().join("test.db");
    let tm = TaskManager::new(db_path.to_str().unwrap());
    tm.truncate();
 
    assert_eq!(tm.get_tasks().len(), 0);
    
    let task1 = tm.create_task(
        String::from("run"),
        String::from("run around the block")
    );

    assert_eq!(tm.get_task(task1.id).unwrap(), task1);    
    
    let task_entry = tm.create_task_entry(task1.id, String::from("did 2 laps")
        .into_bytes());

    assert_eq!(tm.get_task_entry(task1.id, task_entry.id).unwrap(), task_entry);
    assert_eq!(tm.get_task_entries(task1.id).len(), 1);
    assert_eq!(tm.get_task_entries(task1.id).first().unwrap(), &task_entry);
    
    let task2 = tm.create_task(
        String::from("swim"),
        String::from("swim in the pool")
    );

    let swim_entry = tm.create_task_entry(task1.id, String::from("went in fast lane")
        .into_bytes());

    assert_eq!(tm.get_all_entries().len(), 2); 
}


#[test]
fn test_truncate() {

    let db_path = TempDir::new().unwrap().path().join("test.db");
    let tm = TaskManager::new(db_path.to_str().unwrap());
    
    tm.truncate(); 
    assert_eq!(tm.get_tasks().len(), 0);
    
    let task1 = tm.create_task(
        String::from("run"),
        String::from("run around the block")
    );
    let task2 = tm.create_task(
        String::from("swim"),
        String::from("swim in the pool")
    );

    tm.create_task_entry(task1.id, String::from("did 2 laps")
        .into_bytes());
    tm.create_task_entry(task1.id, String::from("went in fast lane")
        .into_bytes());
 
    tm.truncate();
    assert_eq!(tm.get_all_entries().len(), 0); 
    assert_eq!(tm.get_tasks().len(), 0); 
}

