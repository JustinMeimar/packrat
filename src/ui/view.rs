/// view.rs

use std::io;
use crate::model::task_entry::TaskEntry;
use crate::ui::state::{EntryViewState, MainViewState, TaskViewState,
                       CreateTaskViewState, CreateEntryViewState};
use crate::ui::render::renderable::Renderable;
use crate::ui::render::render_create::FormRenderable;
use crate::model::task::Task;

///////////////////////////////////////////////////////////

/// TODO: Fix some confusing naming
#[derive(Debug, PartialEq)]
pub enum View {
    MainView(MainViewState),                // list of tasks
    TaskView(TaskViewState),                // list of task entries
    EntryView(EntryViewState),              // view an entry (vim)
    CreateTaskView(CreateTaskViewState),    // form for new Task
    CreateEntryView(CreateEntryViewState),  // form for new TaskEntry
}

#[derive(Debug, PartialEq)]
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

        let view_state = MainViewState::new();
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
                Some(View::CreateTaskView(cts)) => cts.render()?,
                Some(View::CreateEntryView(ces)) => ces.render()?,
                _ => panic!("This is a packrat bug!")
            };
            
            // dispatch next view based on transition
            match transition {
                Transition::Push(v) => self.view_stack.push(v),
                Transition::Pop => {
                    if self.view_stack.len() == 1 {
                        break;
                    }
                    self.view_stack.pop();
                },
                Transition::Quit => break,
                Transition::Stay => continue,
                _ => panic!("This is a packrat bug!")
            }
        }
        Ok(())
    }

}

