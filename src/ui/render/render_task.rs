use std::io;
use tui::layout::Rect;
use crate::model::store::TaskStore;
use crate::ui::view::Transition;
use crate::ui::state::{TaskViewState, EntryViewState, CreateViewState};
use crate::ui::widgets::{list_factory, control_widget, map_list_styles};
use crate::model::task::Task;
use crate::model::task_entry::TaskEntry;
use std::time::Instant;
use tui::layout::{Constraint, Layout};
use crossterm::event::{Event, KeyCode, KeyEvent};
use crate::ui::view::View;
use crate::ui::render::renderable::{
    Renderable, ControlOption, AnyWidget,
    render_view, default_controls 
};

///////////////////////////////////////////////////////////

impl Renderable for TaskViewState {
    
    // Draw the View on the terminal
    fn render(&mut self) -> io::Result<Transition> {
        render_view(self, Self::controler)
    }

    // Create the chunks that widgets will render ontop of 
    fn chunks(&self, frame: Rect) -> Vec<Rect> {        
        Layout::default()
            .constraints([Constraint::Length(3), Constraint::Max(50)].as_ref())
            .split(frame)
    }

    // Render the main view controls and the list of tasks
    fn widgets(&mut self) -> io::Result<Vec<AnyWidget>> {
           
        let task_items: Vec<TaskEntry> = TaskStore::instance()
            .get_prefix(TaskEntry::key_task(self.task.id))
            .unwrap(); 

        let task_styles = map_list_styles(&task_items, self.selector.idx); 
        let task_widget = list_factory(task_items, task_styles, "Tasks");
        
        Ok(vec![control_widget(), task_widget])
    } 

    /// Check the poll interval 
    fn poll(&mut self) {
         if self.last_poll_time.elapsed() >= self.poll_interval {
            self.update();
            self.last_poll_time = Instant::now();
        }
    }
    
    /// Refresh the task entries
    fn update(&mut self) {
        self.items = TaskStore::instance()
            .get_prefix(TaskEntry::key_task(self.task.id))
            .unwrap(); 
        self.selector.max_idx = self.items.len();
    }

    fn controler(&mut self) -> Transition {

        match default_controls(&mut self.selector) {
    
            // A default case was handled
            ControlOption::T(t) => t,
            
            // A custom case occurred
            ControlOption::E(e) => { 
                match e {  
                    /// What to do on "new"
                    Event::Key(KeyEvent { code: KeyCode::Char('n'), .. }) 
                        => {
                            Transition::Push(
                                View::CreateView(
                                    CreateViewState::new(
                                        Task::new("New Task", "Task Description")
                                    )
                                )
                            )
                        } 
                    /// What to do on "select"
                    Event::Key(KeyEvent { code: KeyCode::Char('s') | KeyCode::Enter, .. })
                        => {
                            let item = self.items[self.selector.idx].clone();
                            Transition::Push(View::EntryView(EntryViewState::new(item)))
                        } 
                    _ => Transition::Stay
                }
            }
        }
    }
}

