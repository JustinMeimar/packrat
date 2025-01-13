/// view.rs

use std::io;
use crate::model::convert::Storable;
use crate::ui::state::*;
use crate::ui::render::renderable::Renderable;
use crate::ui::render::render_create::FormRenderable;
use crate::model::task::Task;
use std::fmt::{Display, Debug};

///////////////////////////////////////////////////////////

/// TODO: Fix some confusing naming
#[derive(Debug)]
pub enum View {
    MainView(MainViewState),                // list of tasks
    TaskView(TaskViewState),                // list of task entries
    EntryView(EntryViewState),              // view an entry (vim)
    CreateTaskView(CreateTaskViewState),    // form for new Task
    PopUpView(PopUpViewState),

    // dynamic views
    ConfigView(Box<dyn DynView>),
    DeleteView(Box<dyn DynView>), // type erased delete view 
}

// Erase the type of DeleteViewState<T> with a wrapper trait that dynamically
// dispatches rendering. This lets us keep the View enum non-generic.
pub trait DynView: Renderable + Debug {}

/// Implement the wrapper trait 
impl<T: Storable + Debug + 'static> DynView for DeleteViewState<T> {}
impl<T: Storable + Debug + 'static> DynView for ConfigViewState<T> {}

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
                Some(View::ConfigView(cs))      => cs.render()?,
                Some(View::PopUpView(ps))       => ps.render()?,  
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

