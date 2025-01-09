use std::io;
use std::io::{Stdout, Write, Read};
use std::fs::File;
use crossterm::execute;
use tui::layout::Rect;
use tui::{ 
    text::{Span, Spans},
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Table, Widget, Row, Cell},
    Terminal,
};
use tempfile::NamedTempFile;
use crate::model::convert::Storable;
use crate::model::store::TaskStore;
use crate::ui::view::Transition;
use crate::ui::control::UserAction;
use crate::ui::state::{TaskViewState, MainViewState, EntryViewState};
use crate::ui::control::Controlable;
use crate::ui::widgets::{
    term_default_layout,
    term_user_action_list,
    list_factory,
    table_factory,
};
use tui::buffer::Buffer;
use super::state::{CreateViewState, SelectionState};
use crate::model::task::Task;
use crate::model::task_entry::TaskEntry;
use std::process::Command;
use std::time::{Duration, Instant};
use crossterm::event::{self, Event, KeyEvent};

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
          
        let control_items = UserAction::all(); 
        let control_widget = table_factory(vec![control_items], "Controls");
        
        let task_items: Vec<Task> = TaskStore::instance().get_prefix(Task::key_all()).unwrap(); 
        /// I work so had to minify, then the styling takes up half the lines...grrr
        /// Potentially, list_factory can take a lambda and we can just copy over select idx
        let task_styles = task_items.iter().enumerate().map(|(i, t)| {
            if i == self.selector.idx {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            }
        }).collect();

        let task_widget = list_factory(task_items, task_styles, "Tasks");
        
        Ok(vec![control_widget, task_widget])
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
        let transition = loop {

            // poll for updates
            self.poll();
            
            // divide the screen, get the widgets and draw 
            terminal.draw(|f| {
                let sizes = self.chunks(f.size()); 
                let widgets = self.widgets().unwrap(); 
                widgets.into_iter().enumerate().for_each(|(i, w)| {
                    f.render_widget(w, sizes[i]); 
                });
            });          

            // listen for keyboard events
            let transition = self.control();
            if transition != Transition::Stay {
                break transition;
            }
        };
        
        render_view_teardown(&mut terminal);
        Ok(transition)
    } 
}

impl Renderable for TaskViewState {

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
          
        let control_items = UserAction::all(); 
        let control_widget = table_factory(vec![control_items], "Controls");
        
        let task_items: Vec<TaskEntry> = TaskStore::instance()
            .get_prefix(TaskEntry::key_task(self.task.id))
            .unwrap(); 
        
        /// I work so had to minify, then the styling takes up half the lines...grrr
        /// Potentially, list_factory can take a lambda and we can just copy over select idx
        let task_styles = task_items.iter().enumerate().map(|(i, t)| {
            if i == self.selector.idx {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            }
        }).collect();

        let task_widget = list_factory(task_items, task_styles, "Tasks");
        
        Ok(vec![control_widget, task_widget])
    } 

    // Draw the View on the terminal
    fn render(&mut self) -> io::Result<Transition> {

        let mut terminal = render_view_startup()?;
        let transition = loop {

            // poll for updates
            self.poll();
            
            // divide the screen, get the widgets and draw 
            terminal.draw(|f| {
                let sizes = self.chunks(f.size()); 
                let widgets = self.widgets().unwrap(); 
                widgets.into_iter().enumerate().for_each(|(i, w)| {
                    f.render_widget(w, sizes[i]); 
                });
            });          

            // listen for keyboard events
            let transition = self.control();
            if transition != Transition::Stay {
                break transition;
            }
        };
        
        render_view_teardown(&mut terminal);
        Ok(transition)
    }

    /// Check the poll interval 
    fn poll(&mut self) {
         if self.last_poll_time.elapsed() >= self.poll_interval {
            self.update();
            self.last_poll_time = Instant::now();
        }
    }

    fn update(&mut self) {
        self.items = TaskStore::instance()
            .get_prefix(TaskEntry::key_task(self.task.id))
            .unwrap(); 
        self.selector.max_idx = self.items.len();
    }

    // fn update(&mut self) { 
    //     // poll new items
    //     self.items = TaskStore::instance()
    //         .get_prefix(Task::key_all())
    //         .unwrap(); 
    //         
    //     // update selector
    //     self.selector.max_idx = self.items.len();
    // }

    // fn render(&mut self) -> io::Result<Transition> {
    //     
    //     let mut terminal = render_view_startup()?;
    //     let mut transition = Transition::Stay; 
    //     let poll_interval = Duration::from_millis(10);
    //     let mut last_poll_time = Instant::now();
    //
    //     loop { 
    //         if last_poll_time.elapsed() >= poll_interval {
    //             self.update();
    //             last_poll_time = Instant::now();
    //         }
    //
    //         let widgets = vec![
    //             term_user_action_list(),
    //             term_user_task_entries_list(&self.items, self.selector.idx)
    //         ];
    //
    //         draw_widgets(&mut terminal, widgets);
    //
    //         transition = self.control();
    //         match transition {
    //             Transition::Stay => continue,
    //             _ => break
    //         }
    //     }
    //     
    //     render_view_teardown(&mut terminal); 
    //     return Ok(transition);
    // }
}

impl<T: Storable> Renderable for CreateViewState<T> {
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

///////////////////////////////////////////////////////////

fn render_view_startup() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    
    // Flush stdout
    let mut stdout = std::io::stdout();
    execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;

    // Initialize the terminal
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    crossterm::terminal::enable_raw_mode()?;

    Ok(terminal)
}

fn render_view_teardown(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    crossterm::terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), crossterm::terminal::LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

pub fn term_mixed_layout(n: usize) -> Layout {
    let constraints: Vec<Constraint> = vec![Constraint::Length(3); n];
    Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints.as_slice())
}


// Wrapper for the types of widgets we will be rendering
// Enables a heterogenous Vec<WidgetType>
enum WidgetType {
    List(List<'static>),
    Table(Table<'static>),
}

fn render_widgets_enum(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    widgets: Vec<WidgetType>
) -> io::Result<()> {
    terminal.draw(|f| {
        let n_widgets = widgets.len();
        let chunks = term_mixed_layout(n_widgets).split(f.size());
        widgets.into_iter().enumerate().for_each(|(i, widget)| {
            match widget {
                WidgetType::List(list) => f.render_widget(list, chunks[i]),
                WidgetType::Table(table) => f.render_widget(table, chunks[i]),
            }
        });
    })?;
    Ok(())
}

// fn render_widgets(terminal: &mut TerminalTy, )
fn draw_widgets(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    widgets: Vec<List<'static>>
) -> io::Result<()> {
    
    terminal
        .draw(|f| {
            let chunks =    term_default_layout().split(f.size());
            widgets.iter().enumerate().for_each(|(i, w)| {
                f.render_widget(w.clone(), chunks[i]);
            });
        })
        .unwrap();
    Ok(())
}

fn term_user_task_list(tasks: &Vec<Task>,selection: &SelectionState) -> List<'static> {
    
    let task_list: Vec<ListItem> = tasks
        .iter()
        .enumerate()
        .map(|(i, task)| style_list_item(&task.to_string(), selection.idx, i))
        .collect();


    List::new(task_list)
        .block(Block::default().title("Tasks").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
}

fn term_user_task_entries_list(tasks: &Vec<TaskEntry>, idx: usize) -> List<'static> {
    
    let task_list: Vec<ListItem> = tasks
        .iter()
        .enumerate()
        .map(|(i, entry)| style_list_item(&entry.to_string(), idx, i))
        .collect();


    List::new(task_list)
        .block(Block::default().title("Tasks").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
}

fn style_list_item(
    item_text: &str, // Accept a string slice
    selection_idx: usize,
    map_idx: usize,
) -> ListItem<'static> {
    let style = if selection_idx == map_idx {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    ListItem::new(Spans::from(Span::styled(item_text.to_string(), style)))
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
