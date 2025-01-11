/// view.rs

use std::io;
use crate::ui::state::*;
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
    DeleteView(DeleteViewState<Task>),      // delete a task 
    EditView(EditViewState<Task>),          // edit a task
    CreateTaskView(CreateTaskViewState),    // form for new Task
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

            let transition = match self.view_stack.last_mut() { 
                
                // render the view and retrieve next transition
                Some(View::MainView(ms))        => ms.render()?,
                Some(View::TaskView(ts))        => ts.render()?,
                Some(View::EntryView(es))       => es.render()?,
                Some(View::CreateTaskView(cs))  => cs.render()?,
                Some(View::DeleteView(ds))      => ds.render()?,
                
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

