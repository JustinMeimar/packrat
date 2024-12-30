use std::fmt;
use std::thread;
use std::time::Duration;
use std::io::{self, Write, Stdout};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal,
};

use tui::{ 
    text::{Span, Spans, Text},
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

use crate::model::task::Task;
use crate::model::task::TaskManager;
use crate::ui::terminal::{TerminalState, UserAction};

///////////////////////////////////////////////////////////

pub fn render_main_view(state: &mut TerminalState) -> UserAction { 
    
    // set the number of selection options for main view
    state.select_n = state.db.get_tasks().len();
    
    // render indefinitely
    loop {
        if let Ok(action) = _render_view(
            state,
            draw_main_view,
            control_handler_main
        ) {
            return action;
        }
    }
}

pub fn render_task_view(state: &mut TerminalState) -> UserAction {
    loop {
        if let Ok(action) = _render_view(
            state,
            draw_task_view,
            control_handler_main
        ) {
            return action;
        }
    }      
}

///////////////////////////////////////////////////////////

fn _render_view<F, G>(
    state: &mut TerminalState,
    mut draw_fn: F,
    mut control_fn: G
) -> Result<UserAction, std::io::Error>

where F: FnMut(
            &mut Terminal<CrosstermBackend<Stdout>>,
            &mut TerminalState
        ),
      G: FnMut(
            &mut TerminalState
        ) -> UserAction
{ 
    // flush stdout
    let mut stdout = std::io::stdout();
    execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;

    // intit the terminal
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    crossterm::terminal::enable_raw_mode()?;
    
    let mut action = UserAction::Back;

    loop {
        draw_fn(&mut terminal, state);  
        action = control_fn(state);
        match action {
            UserAction::Select | UserAction::Back | UserAction::Quit => break,
            _ => continue
        };
    }
    
    // cleanup
    crossterm::terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), crossterm::terminal::LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(action)
}

fn _sleep(time: u64) {
    thread::sleep(Duration::from_secs(time));
}

//////////////////////////////////////////////////////////////////////////////////

fn draw_main_view(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    state: &mut TerminalState,
) {
    let widgets = vec![
        term_user_action_list(),
        term_user_task_list(state),
    ];
    terminal
        .draw(|f| {
            let chunks = term_default_layout().split(f.size());
            widgets.iter().enumerate().for_each(|(i, w)| {
                f.render_widget(w.clone(), chunks[i]);
            });
        })
        .unwrap();
}

fn draw_task_view(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    state: &mut TerminalState,
) {
    let widgets = vec![
        term_user_action_list(),
        term_user_task_list(state),
    ];
    terminal
        .draw(|f| {
            let chunks = term_default_layout().split(f.size());
            widgets.iter().enumerate().for_each(|(i, w)| {
                f.render_widget(w.clone(), chunks[i]);
            });
        })
        .unwrap();
}

fn control_handler_main(state: &mut TerminalState) -> UserAction {
    
    match event::read().unwrap() {  
        
        Event::Key(KeyEvent { code: KeyCode::Char('q') | KeyCode::Char('e'), .. }) 
            => UserAction::Quit, 
        
        Event::Key(KeyEvent { code: KeyCode::Char('s') | KeyCode::Enter, .. })
            => UserAction::Select, 
        
        Event::Key(KeyEvent { code: KeyCode::Char('j') | KeyCode::Down, .. })
            => { 
                state.select_idx = (state.select_idx + state.select_n + 1) % state.select_n;
                UserAction::None
            }, 
        
        Event::Key(KeyEvent { code: KeyCode::Char('k') | KeyCode::Up, .. })
            => { 
                state.select_idx = (state.select_idx + state.select_n - 1) % state.select_n;
                UserAction::None
            }, _ 
            => UserAction::None,
    }
}

///
///
fn term_default_layout() -> Layout {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
}

///
///
fn term_user_action_list() -> List<'static> {

    let items: Vec<ListItem> = UserAction::all()
        .iter()
        .map(|x| ListItem::new(format!(" > {:?}", x)))
        .collect();

    List::new(items)
        .block(Block::default().title("Controls").borders(Borders::ALL))
        .style(Style::default().fg(Color::Gray))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")   
}

///
/// 
fn term_user_task_list(state: &mut TerminalState) -> List<'static> {
    
    let task_list: Vec<ListItem> = state.db.get_tasks()
        .iter()
        .enumerate()
        .map(|(i, task)| style_list_item(&task.name, state.select_idx, i))
        .collect();


    List::new(task_list)
        .block(Block::default().title("Tasks").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
}

///
/// 
// fn term_task_entries_list(task: &Task, entry_idx: usize) -> List<'static> {
//     
//     // let task_entries: Vec<ListItem> = task.get_entries()
//     //     .iter()
//     //     .enumerate()
//     //     .map(|(i, entry)| style_list_item(entry.date.to_string(), entry_idx, i))
//     //     .collect();
//     //
//     // List::new(task_entries)
//     //     .block(Block::default().title("Entries").borders(Borders::ALL))
//     //     .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
// }

///

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

