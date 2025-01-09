use std::io;
use std::io::{Stdout, Write, Read};
use std::fs::File;
use crossterm::execute;
use tui::layout::Rect;
use tempfile::NamedTempFile;
use crate::model::convert::Storable;
use crate::model::store::TaskStore;
use crate::ui::view::Transition;
use crate::ui::state::{TaskViewState, MainViewState, EntryViewState};
use crate::ui::control::Controlable;
use crate::ui::widgets::{list_factory, control_widget, map_list_styles};
use tui::buffer::Buffer;
use super::state::CreateViewState;
use crate::model::task::Task;
use crate::model::task_entry::TaskEntry;
use std::process::Command;
use std::time::Instant;
use tui::{ 
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, Paragraph, Table, Widget},
    Terminal,
};
///////////////////////////////////////////////////////////

type TerminalTy = Terminal<CrosstermBackend<Stdout>>;

/// A way to keep a vector of types that implement the trait Widget is to wrap said
/// types in an enum. We use a lifetime parameter because the widget types such as
/// List and Table require so.
#[derive(Clone)]
pub enum AnyWidget<'a> {
    List(List<'a>),
    Table(Table<'a>),
}

impl<'a> Widget for AnyWidget<'a> {

    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            AnyWidget::List(list) => list.render(area, buf),
            AnyWidget::Table(table) => table.render(area, buf),
        }
    }
}

pub trait Renderable { 
    
    /// Main interface to render a view on a layout 
    fn render(&mut self) -> io::Result<Transition>; 

    /// Refresh dispaly items, default none
    fn update(&mut self) {}
    
    /// Set time interval to trigger updates
    /// TODO: What do I need to do to let a default impl work when it must reference self?
    fn poll(&mut self) {} 
    
    /// Return a division of the current frame into chunks 
    fn chunks(&self, frame: Rect) -> Vec<Rect> { Layout::default().split(frame) }
    
    /// What we actually drawin 
    fn widgets(&mut self) -> io::Result<Vec<AnyWidget>> { Ok(vec![]) } 
 
    // Handle the keyboard controls for a view
    fn controler(&mut self) -> Transition { Transition::Stay }
}

///////////////////////////////////////////////////////////

impl Renderable for MainViewState {
       
    // Create the chunks that widgets will render ontop of 
    fn chunks(&self, frame: Rect) -> Vec<Rect> {        
        Layout::default()
            .constraints([
                Constraint::Length(3),
                Constraint::Max(50) // fill remaining height
            ].as_ref())
            .split(frame)
    }
    
    // Render the main view controls and the list of tasks
    fn widgets(&mut self) -> io::Result<Vec<AnyWidget>> {
                  
        let task_items: Vec<Task> = TaskStore::instance().get_prefix(Task::key_all()).unwrap(); 
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
    
    /// What to do during each poll interval
    fn update(&mut self) { 
        // poll new items
        self.items = TaskStore::instance()
            .get_prefix(Task::key_all())
            .unwrap(); 
            
        // update selector
        self.selector.max_idx = self.items.len();
    }
    
    // Draw the View on the terminal
    fn render(&mut self) -> io::Result<Transition> {

        let mut terminal = render_view_startup()?;
        let transition = render_view(self, &mut terminal, Self::control);        
        render_view_teardown(&mut terminal);
        transition
    } 
}

impl Renderable for TaskViewState {
    
    // Draw the View on the terminal
    fn render(&mut self) -> io::Result<Transition> {

        let mut terminal = render_view_startup()?;
        let transition = render_view(self, &mut terminal, Self::control);  
        render_view_teardown(&mut terminal);
        transition
    }

    // Create the chunks that widgets will render ontop of 
    fn chunks(&self, frame: Rect) -> Vec<Rect> {        
        Layout::default()
            .constraints([
                Constraint::Length(3),
                Constraint::Max(50) // fill remaining height
            ].as_ref())
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
}

///////////////////////////////////////////////////////////

pub fn render_view<R>(
    state: &mut R,
    terminal: &mut Terminal<impl Backend>,
    control_handler: impl Fn(&mut R) -> Transition,
) -> io::Result<Transition>
where
    R: Renderable,
{
    loop {
        state.poll();
        terminal.draw(|f| {
            let chunks = state.chunks(f.size());
            let widgets = state.widgets().unwrap();
            widgets.into_iter().enumerate().for_each(|(i, w)| {
                f.render_widget(w, chunks[i]);
            });
        })?;

        let transition = control_handler(state);
        if transition != Transition::Stay {
            return Ok(transition);
        }
    }
}

fn render_view_startup() -> io::Result<TerminalTy> { 
    // Flush stdout
    let mut stdout = std::io::stdout();
    execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;

    // Initialize the terminal
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    crossterm::terminal::enable_raw_mode()?;

    Ok(terminal)
}

fn render_view_teardown(terminal: &mut TerminalTy) -> io::Result<()> {
    crossterm::terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), crossterm::terminal::LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

///////////////////////////////////////////////////////////

impl Renderable for EntryViewState {
    
    // A hacky, happy-path implementation for now
    fn render(&mut self) -> io::Result<Transition> {
        
        // get the contents of selected task entry
        let content = &self.task_entry.content;
        
        // open a temporary file 
        let mut tmp_file = NamedTempFile::new()
            .map_err(|e| format!("Failed to create tmpfile: {}", e))
            .unwrap();
        
        // write the contents into the file 
        tmp_file
            .write_all(&content)
            .map_err(|e| format!("Failed to write to temp file: {}", e));

        // open the editor
        let status = Command::new("nvim")
            .arg(tmp_file.path())
            .status()
            .expect("Failed to open editor");

        if !status.success() {
            eprintln!("Neovim exited with an error."); 
        }

        // read the contents back
        let mut content_updated = String::new();
        File::open(&tmp_file)?
            .read_to_string(&mut content_updated)?;

        // synchronize the updates
        self.task_entry.content = content_updated.into_bytes();  
        TaskStore::instance().put(self.task_entry.clone());
        
        Ok(Transition::Pop)
    }
}

///////////////////////////////////////////////////////////

impl<T: Storable> Renderable for CreateViewState<T> {

    /// TODO: This implementation is fully rolled out for first iteration, needs
    /// to be factored into the widget library
    fn render(&mut self) -> io::Result<Transition> {
        
        let mut terminal = render_view_startup()?;
        let mut transition = Transition::Stay; 

        let result = loop {
            terminal.draw(|f| {
                let size = f.size();
                let modal_width = size.width / 3 * 2;
                let modal_height = 10;

                let modal_area = tui::layout::Rect::new(
                    (size.width - modal_width) / 2,
                    (size.height - modal_height) / 2,
                    modal_width,
                    modal_height,
                );
                
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ])
                    .split(modal_area);

                let modal_block = Block::default()
                    .title("Create New Task")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Gray));
                f.render_widget(modal_block, modal_area);

                let title_widget = Paragraph::new(self.inputs[0].as_str())
                    .block(
                        Block::default()
                            .title("Task Title")
                            .borders(Borders::ALL)
                            .style(if self.active_input == 0 {
                                Style::default().fg(Color::Yellow)
                            } else {
                                Style::default()
                            }),
                    );

                let desc_widget = Paragraph::new(self.inputs[1].as_str())
                    .block(
                        Block::default()
                            .title("Task Description")
                            .borders(Borders::ALL)
                            .style(if self.active_input == 1 {
                                Style::default().fg(Color::Yellow)
                            } else {
                                Style::default()
                            }),
                    );

                f.render_widget(title_widget, chunks[0]);
                f.render_widget(desc_widget, chunks[1]);
            })?;
            
            transition = self.control();
            match transition {
                Transition::Stay => continue,
                _ => break
            } 
        };

        // Ensure the terminal is properly torn down before returning
        render_view_teardown(&mut terminal)?;
        Ok(transition)
    }
}

