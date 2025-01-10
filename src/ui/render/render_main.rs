use std::io;
use tui::layout::Rect;
use crate::model::store::TaskStore;
use crate::ui::view::Transition;
use crate::ui::state::{TaskViewState, MainViewState, CreateTaskViewState};
use crate::ui::widgets::{list_factory, control_widget, map_list_styles, task_table};
use crate::model::task::Task;
use std::time::Instant;
use tui::layout::{Constraint, Layout};
use crossterm::event::{Event, KeyCode, KeyEvent};
use crate::ui::view::View;
use crate::ui::render::renderable::{
    Renderable, ControlOption, AnyWidget,
    render_view, default_controls 
};

///////////////////////////////////////////////////////////

impl Renderable for MainViewState {
       
    /// Create the chunks that widgets will render ontop of 
    fn chunks(&self, frame: Rect) -> Vec<Rect> {        
        Layout::default()
            .constraints([Constraint::Length(3),Constraint::Max(50)].as_ref())
            .split(frame)
    }
    
    /// Render the main view controls and the list of tasks
    fn widgets(&mut self) -> io::Result<Vec<AnyWidget>> {
                  
        let task_items: Vec<Task> = TaskStore::instance()
            .get_prefix(Task::key_all())
            .unwrap();
            
        let task_widget = task_table(task_items, self.selector.idx);
        
        Ok(vec![control_widget(), task_widget])
    } 

    /// Check the poll interval 
    fn poll(&mut self) {
         if self.last_poll_time.elapsed() >= self.poll_interval {
            self.update();
            self.last_poll_time = Instant::now();
        }
    }
    
    /// What to do during each poll interval
    fn update(&mut self) { 
        // poll new items
        self.items = TaskStore::instance()
            .get_prefix(Task::key_all())
            .unwrap(); 
            
        // update selector
        self.selector.max_idx = self.items.len();
    }
    
    /// Draw the View on the terminal
    fn render(&mut self) -> io::Result<Transition> {
        return render_view(self, Self::controler);
    } 
    
    /// Handle the user inputs
    fn controler(&mut self) -> Transition {

        match default_controls(&mut self.selector) {
    
            // A default case was handled
            ControlOption::T(t) => t,
            
            // A custom case occurred
            ControlOption::E(e) => { 
                match e { 
                    /// What to do on "edit"
                    Event::Key(KeyEvent { code: KeyCode::Char('e'), .. }) 
                        => {
                            Transition::Stay
                            // let item: Task = self.items[self.selector.idx].clone();
                            // Transition::Push(
                            //     View::CreateView(
                            //         CreateViewState::new(
                            //             item.clone()
                            //         )
                            //     )
                            // )
                        } 
                    /// What to do on "new"
                    Event::Key(KeyEvent { code: KeyCode::Char('n'), .. }) 
                        => {
                            Transition::Push(
                                View::CreateTaskView(
                                    CreateTaskViewState::new(
                                        Task::new("New Task", "Task Description")
                                    )
                                )
                            )
                        } 
                    /// What to do on "select"
                    Event::Key(KeyEvent { code: KeyCode::Char('s') | KeyCode::Enter, .. })
                        => {
                            let item = self.items[self.selector.idx].clone();
                            Transition::Push(
                                View::TaskView(
                                    TaskViewState::new(item)
                                )
                            )
                        }
                    _ => Transition::Stay
                }
            }
        }
    }
}

