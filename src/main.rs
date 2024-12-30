use packrat::model::task::{Task, TaskManager};
use packrat::ui::terminal;
use std::io;

fn main() -> Result<(), io::Error>  {
    
    // open database
    let db_path = "./scratch/patrack.db";
    let db = TaskManager::new(db_path);

    // move ownership to terminal state
    terminal::start(db);

    Ok(())
}

