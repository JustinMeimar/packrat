use std::io;
use crate::model::task::{TaskManager, Task, TaskEntry};
use crate::ui::state::{EntryViewState, MainViewState, TaskViewState};
use crate::ui::render::Renderable;
// use crate::ui::render::{TaskViewState, MainViewState, EntryViewState};

use std::process::Command;
use std::path::Path;
use std::thread;
use std::time::Duration;

///////////////////////////////////////////////////////////
///
///////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum View {
    MainView(MainViewState),
    TaskView(TaskViewState),
    EntryView(EntryViewState),
}

#[derive(Debug)]
pub enum Transition {
    Push(View),
    Pop,
    Quit,
    Stay,
}

pub struct App {
    pub view_stack: Vec<View>
}

////////////////////////////////////////////////////////////
///
////////////////////////////////////////////////////////////

impl App {
    
    pub fn new() -> Self {

        let mut view_state = MainViewState::new();
        let mut view_stack = Vec::new();
        view_stack.push(View::MainView(view_state));

        App { view_stack }
    }

    pub fn run(&mut self) -> io::Result<()> {
        
        loop {

            // render the view and retrieve next transition
            let transition = match self.view_stack.last_mut() { 
                Some(View::MainView(ms)) => ms.render()?,
                Some(View::TaskView(ts)) => ts.render()?,
                Some(View::EntryView(es)) => es.render()?,
                _ => panic!("This is a packrat bug!")
            };
            
            println!("Got transition: {:?}", transition);

            // dispatch next view based on transition
            match transition {
                Transition::Push(v) => self.view_stack.push(v),
                Transition::Pop => { self.view_stack.pop(); },
                Transition::Quit => break,
                Transition::Stay => continue,
                _ => panic!("This is a packrat bug!")
            }
        }
        Ok(())
    }

}

fn _sleep(time: u64) {
    thread::sleep(Duration::from_secs(time));
}

////////////////////////////////////////////////////////////
///
////////////////////////////////////////////////////////////

pub fn test_render_main_view(state: &mut MainViewState, db: &TaskManager) -> Transition {
    
    let max_idx = db.get_tasks().len();

    loop {
        println!("Rendering the main view... ({:?} tasks)", max_idx);
        // _sleep(1);
        // state.idx = 1;
        // let selected_task = db.get_tasks()[state.idx].clone();

        // return Transition::Push(
        //     View::TaskView(TaskViewState::new(selected_task))
        // );
    }
}


pub fn test_render_task_view(state: &mut TaskViewState, db: &TaskManager) -> Transition {
    
    let task = state.task.clone();
    let max_idx = db.get_task_entries(task.id).len();
    loop {
        // println!("Rendering the task view... ({:?} entries)", max_idx);
        // _sleep(1); 
    }
}

