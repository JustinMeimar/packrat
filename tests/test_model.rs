use packrat::model::convert::Storable;
use packrat::model::task::Task;
use packrat::model::task_entry::TaskEntry;
use packrat::model::store::TaskStore;
use tempfile::TempDir;

fn get_empty_db() -> TaskStore {
    let temp_dir = TempDir::new().unwrap();
    TaskStore::new(temp_dir.path().join("test.db").to_str().unwrap())
}

fn fill_db_tasks_random(ts: &TaskStore, n: usize) { 
    // fill the store
    ts.put(Task::new("Walk Dog", "Walk buddy around the block")).unwrap();
    ts.put(Task::new("Learn Rust", "Harness crab energy")).unwrap(); 
}

fn fill_db_entries_random(ts: &TaskStore, n: usize) { 
    // fill the store 
    ts.put(Task::new("Walk Dog", "Walk buddy around the block")).unwrap();
    ts.put(Task::new("Learn Rust", "Harness crab energy")).unwrap(); 
}

#[test]
fn test_task() {
    
    // create task
    let task = Task::new(String::from("clean"), String::from("do the dishes"));
    assert_eq!(task.name, "clean");
    assert_eq!(task.desc, "do the dishes");
        
    // serialize and deserialize
    let ser_task = task.to_bytes().unwrap();
    let de_task = Task::from_bytes(&ser_task).unwrap();
    assert_eq!(de_task, task);
}

#[test]
fn test_store_put() {
    
    let ts = get_empty_db(); 
    
    // put in store
    let task1 = ts.put(Task::new("Walk Dog", "Walk buddy around the block")).unwrap();
    let task2 = ts.put(Task::new("Learn Rust", "Harness crab energy")).unwrap();
    
    assert_eq!(ts.get_prefix::<Task>(Task::key_all()).unwrap().len(), 2);

    let entry_1 = ts.put(TaskEntry::new(task1.id, "Today I walked the dog"));
    let entry_2 = ts.put(TaskEntry::new(task1.id, "Yesterday I walked buddy twice"));
    
    // get what we put back
    match ts.get::<Task>(task1.to_key()).unwrap() {
        Some(t) => {
            assert_eq!(t.name, task1.name);
            assert_eq!(t.id, task1.id);
            assert_eq!(t, task1);
        }
        None => assert_eq!("", "Task 1 could not be retrieved from store."),
    }

    match ts.get::<Task>(task2.to_key()).unwrap() {
        Some(t) => assert_eq!(task2, t),
        None => assert_eq!("", "Task 1 could not be retrieved from store."),
    }
}

#[test]
fn test_task_manager() {
   
    let ts = get_empty_db();
    assert_eq!(ts.get_prefix::<Task>(Task::key_all()).unwrap().len(), 0);

    fill_db_tasks_random(&ts, 2); 
    assert_eq!(ts.get_prefix::<Task>(Task::key_all()).unwrap().len(), 2); 
    
    fill_db_tasks_random(&ts, 2);
    assert_eq!(ts.get_prefix::<Task>(Task::key_all()).unwrap().len(), 4); 

    ts.truncate();
    assert_eq!(ts.get_prefix::<Task>(Task::key_all()).unwrap().len(), 0); 
}

