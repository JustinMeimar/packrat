/// main.rs

use std::io;
use std::time::Duration;
use std::thread;
use packrat::model::task::Task;
use packrat::model::task_manager::TaskManager;
use packrat::model::task_entry::TaskEntry;
use packrat::model::convert::BytesConvertible;
use packrat::ui::terminal;
use packrat::ui::view;

///////////////////////////////////////////////////////////

fn add_dummy_tasks() {

    let tm = TaskManager::instance().lock().unwrap();
    
    let task1 = tm.create_task(String::from("walk dog"), String::from("walking the block"));
    let task2 = tm.create_task(String::from("run"), String::from("run around"));
    let task3 = tm.create_task(String::from("rust"), String::from("avoid crabs"));
    
    tm.create_task_entry(task1.id, String::from("I walked today").into_bytes());
    tm.create_task_entry(task1.id, String::from("A second walk").into_bytes());
    tm.create_task_entry(task1.id, String::from("A second walk").into_bytes());
    tm.create_task_entry(task1.id, String::from("The dog is tired").into_bytes());
    
    tm.debug_dump();
}

fn main() -> Result<(), io::Error>  {
     
    // remove previous dummy data
    TaskManager::instance()
        .lock()
        .unwrap()
        .truncate(); 

    // populate some tasks
    add_dummy_tasks(); 
 
    // move ownership to the app
    let mut app = view::App::new();
    
    // run
    app.run();

    Ok(())
}

