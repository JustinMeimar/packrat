/// main.rs
use std::io;
use packrat::model::task::Task;
use packrat::model::store::TaskStore;
use packrat::model::task_entry::TaskEntry;
use packrat::ui::view;

///////////////////////////////////////////////////////////

fn add_dummy_tasks() {
    
    let ts = TaskStore::instance();

    let task1 = ts.put(Task::new("Walk Dog", "Walk buddy around the block")).unwrap();
    let task2 = ts.put(Task::new("Learn Rust", "Harness crab energy")).unwrap();
    let task3 = ts.put(Task::new("Meditate", "Just sit down and do nothing!")).unwrap();
    
    let entry_1 = ts.put(TaskEntry::new(task1.id, "Today I walked the dog"));
    let entry_2 = ts.put(TaskEntry::new(task1.id, "Yesterday I walked buddy twice"));
    let entry_3 = ts.put(TaskEntry::new(task2.id, "I learned about .transpose()"));
    
    ts.dump();
}

fn main() -> Result<(), io::Error>  {
        
    // remove previous dummy data
    TaskStore::instance().truncate();
    
    // populate some tasks
    add_dummy_tasks(); 
 
    // move ownership to the app
    let mut app = view::App::new();
    
    // run
    app.run();

    Ok(())
}

