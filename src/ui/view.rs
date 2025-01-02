use std::io;
use crate::model::task::{TaskManager, Task, TaskEntry};
use crate::ui::state::{EntryViewState, MainViewState, TaskViewState};
use crate::ui::render::Renderable;
use std::process::Command;
use std::path::Path;
use std::thread;
use std::time::Duration;

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

