use std::io;
use std::io::{Stdout, Write, Read};
use std::fs::File;
use crossterm::execute;
use tui::{ 
    text::{Span, Spans},
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use tempfile::NamedTempFile;
use crate::model::convert::Storable;
use crate::model::store::TaskStore;
use crate::ui::view::Transition;
use crate::ui::state::{TaskViewState, MainViewState, EntryViewState};
use crate::ui::control::Controlable;
use crate::ui::widgets::{
    term_default_layout,
    term_user_action_list,
};
use super::state::{CreateViewState, SelectionState};
use crate::model::task::Task;
use crate::model::task_entry::TaskEntry;
use std::process::Command;

///////////////////////////////////////////////////////////

pub trait Renderable { 
    
    /// 
    fn render(&mut self) -> io::Result<Transition>;
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

use std::time::{Duration, Instant};
use crossterm::event::{self, Event, KeyEvent};

impl Renderable for MainViewState {

    fn render(&mut self) -> io::Result<Transition> {
        
        let mut terminal = render_view_startup()?;
        let mut transition = Transition::Stay; 
        let poll_interval = Duration::from_millis(100);
        let mut last_poll_time = Instant::now();

        loop {

            if last_poll_time.elapsed() >= poll_interval {
                self.update();
                last_poll_time = Instant::now();
            }

            let widgets = vec![
                term_user_action_list(),
                term_user_task_list(&self.items, &self.selector),
            ];
            
            draw_widgets(&mut terminal, widgets);

            transition = self.control();
            match transition {
                Transition::Stay => continue,
                _ => break
            }
        }
        render_view_teardown(&mut terminal); 
        
        return Ok(transition);
    }
}

impl Renderable for TaskViewState {

    fn render(&mut self) -> io::Result<Transition> {
        
        let mut terminal = render_view_startup()?;
        let mut transition = Transition::Stay; 
        let poll_interval = Duration::from_millis(10);
        let mut last_poll_time = Instant::now();

        loop { 
            if last_poll_time.elapsed() >= poll_interval {
                self.update();
                last_poll_time = Instant::now();
            }

            let widgets = vec![
                term_user_action_list(),
                term_user_task_entries_list(&self.items, self.selector.idx)
            ];

            draw_widgets(&mut terminal, widgets);

            transition = self.control();
            match transition {
                Transition::Stay => continue,
                _ => break
            }
        }
        
        render_view_teardown(&mut terminal); 
        return Ok(transition);
    }
}

impl<T: Storable> Renderable for CreateViewState<T> {
    fn render(&mut self) -> io::Result<Transition> {
        let mut terminal = render_view_startup()?;
        let mut title_input = String::new();
        let mut desc_input = String::new();
        let mut is_title_active = true;

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

                let title_widget = Paragraph::new(title_input.as_str())
                    .block(
                        Block::default()
                            .title("Task Title")
                            .borders(Borders::ALL)
                            .style(if is_title_active {
                                Style::default().fg(Color::Yellow)
                            } else {
                                Style::default()
                            }),
                    );

                let desc_widget = Paragraph::new(desc_input.as_str())
                    .block(
                        Block::default()
                            .title("Task Description")
                            .borders(Borders::ALL)
                            .style(if !is_title_active {
                                Style::default().fg(Color::Yellow)
                            } else {
                                Style::default()
                            }),
                    );

                f.render_widget(desc_widget, chunks[1]);
                f.render_widget(title_widget, chunks[0]);
            })?;

            match crossterm::event::read().unwrap() {
                crossterm::event::Event::Key(event) => match event.code {
                    crossterm::event::KeyCode::Char('q') => {
                        break Ok(Transition::Pop);
                    }
                    crossterm::event::KeyCode::Char(c) => {
                        if is_title_active {
                            title_input.push(c);
                        } else {
                            desc_input.push(c);
                        }
                    }
                    crossterm::event::KeyCode::Backspace => {
                        if is_title_active {
                            title_input.pop();
                        } else {
                            desc_input.pop();
                        }
                    }
                    crossterm::event::KeyCode::Tab => {
                        is_title_active = !is_title_active;
                    }
                    crossterm::event::KeyCode::Enter => {
                        if !title_input.trim().is_empty() && !desc_input.trim().is_empty() {
                            TaskStore::instance().put(
                                Task::new(title_input.clone(), desc_input.clone())
                            );
                            break Ok(Transition::Pop);
                        }
                    }
                    crossterm::event::KeyCode::Esc => {
                        break Ok(Transition::Pop);
                    }
                    _ => {}
                },
                _ => {}
            }
        };

        // Ensure the terminal is properly torn down before returning
        render_view_teardown(&mut terminal)?;
        result
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

pub fn term_mixed_layout() -> Layout {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
}

fn draw_widgets(terminal: &mut Terminal<CrosstermBackend<Stdout>>, widgets: Vec<List<'static>>) {
    
    terminal
        .draw(|f| {
            let chunks = term_default_layout().split(f.size());
            widgets.iter().enumerate().for_each(|(i, w)| {
                f.render_widget(w.clone(), chunks[i]);
            });
        })
        .unwrap();
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

