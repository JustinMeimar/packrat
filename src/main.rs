use packrat::model::task::{Task, TaskManager};
use packrat::ui::terminal;
use std::io;
use std::time::Duration;
use std::thread;

fn add_dummy_tasks(tm: &TaskManager) {

    let task1 = tm.create_task(String::from("walk dog"), String::from("walking the block"));
    let task2 = tm.create_task(String::from("run"), String::from("run around"));
    let task3 = tm.create_task(String::from("rust"), String::from("avoid crabs"));
    
    tm.create_task_entry(task1.id, String::from("I walked today").into_bytes());
    tm.create_task_entry(task1.id, String::from("A second walk").into_bytes());
    tm.create_task_entry(task1.id, String::from("The dog is tired").into_bytes());
    tm.debug_dump();
}

fn main() -> Result<(), io::Error>  {
    
    // open database
    let db_path = "./scratch/patrack.db";
    let db = TaskManager::new(db_path);
    
    // remove previous dummy data
    db.truncate(); 

    // populate some tasks
    add_dummy_tasks(&db); 

    // move ownership to terminal state
    terminal::start(db);

    Ok(())
}

