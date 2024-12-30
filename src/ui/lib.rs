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
use crate::ui::terminal::{TerminalState, UserAction};

///////////////////////////////////////////////////////////

pub fn render_main_view(state: &mut TerminalState) -> UserAction {
    
    loop {
        println!("Rendering main view");
        _sleep(1);
        
    }
    UserAction::Select      
}

pub fn render_task_view(state: &mut TerminalState) -> UserAction {

    loop {
        println!("Rendering task view");
        _sleep(1); 
    }
    UserAction::Select      
}

///////////////////////////////////////////////////////////

// Pulled from previous implementation -- needs some ironing out
fn _render_view<F, G>(
    state: &mut TerminalState,
    mut draw_fn: F,
    mut control_fn: G
) -> Result<(), std::io::Error>

where F: FnMut(
            &mut Terminal<CrosstermBackend<Stdout>>,
            &mut TerminalState
        ),
      G: FnMut(
            &mut TerminalState
        ) -> bool
{ 
    // flush stdout
    let mut stdout = std::io::stdout();
    execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;

    // intit the terminal
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    crossterm::terminal::enable_raw_mode()?;

    loop {
        draw_fn(&mut terminal, state); 
        if !control_fn(state) {
            break;
        } 
    }
    
    // cleanup
    crossterm::terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), crossterm::terminal::LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn _sleep(time: u64) {
    thread::sleep(Duration::from_secs(time));
}

