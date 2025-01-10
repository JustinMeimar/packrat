use std::io;
use tui::layout::Rect;
use crate::model::store::TaskStore;
use crate::ui::view::Transition;
use crate::ui::state::{TaskViewState, EntryViewState, CreateEntryViewState};
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
use crate::log::debug_log;
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
        
        let entries: Vec<TaskEntry> = TaskStore::instance()
            .get_prefix(TaskEntry::key_task(self.task.id))
            .unwrap();


        let all_entries: Vec<TaskEntry> = TaskStore::instance().get_prefix(TaskEntry::key_all()).unwrap();
    
        // let entries = self.task.get_entries();

        debug_log(&format!("{:?}", self.task.id));
        debug_log(&format!("{:?}",
                TaskStore::instance()
                    .get::<Task>(Task::key_task(self.task.id))
                    .unwrap()));

        debug_log(&format!("{:?}", entries));
         
        let task_styles = map_list_styles(&task_items, self.selector.idx); 
        let task_widget = list_factory(task_items, task_styles, self.task.name.clone());
         
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
                    // What to do on "new"
                    Event::Key(KeyEvent { code: KeyCode::Char('n'), .. }) 
                        => {
                            debug_log(&format!("task id: {}", self.task.id)); 
                            // TODO: Decide if we want to create a dialogue box
                            // for new entries, or automatically creating one is sufficient.
                            // For now, just create a entry directly 
                            TaskStore::instance().put(
                                TaskEntry::new(self.task.id, "")
                            );
                            Transition::Stay 
                        } 
                    // What to do on "select"
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

