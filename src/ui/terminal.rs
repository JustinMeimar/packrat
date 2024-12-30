use std::io;
use crate::ui::lib::{render_main_view, render_task_view};
use crate::model::task::TaskManager;

///////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub enum UserAction {
    Select,
    Back,
    Quit,
    None,
}

impl UserAction {
    pub fn all() -> Vec<UserAction> {
        vec![
            UserAction::Select,
            UserAction::Back,
            UserAction::Quit,
            UserAction::None,
        ]
    } 
    pub fn from_index(index: usize) -> Self {
        UserAction::all()[index].clone()
    }
}

enum AppState {
    MainMenu,
    ViewTask,
    Editor,
    Done,
}

pub struct TerminalState { 
    select_idx: usize,
    db: TaskManager,
}

impl TerminalState {
    pub fn new(db: TaskManager) -> Self {
        TerminalState {
            select_idx: 0,
            db,
        }
    }
}

///////////////////////////////////////////////////////////

pub fn start(db: TaskManager) -> Result<(), io::Error> {
    
    let mut app_state = AppState::MainMenu;
    let mut term_state = TerminalState::new(db);

    loop { 
        app_state = match app_state { 
            AppState::MainMenu => run_view_main(&mut term_state)?, 
            AppState::ViewTask => run_view_task(&mut term_state)?, 
            AppState::Done => break,
            _ => break,
        }
    }
    Ok(())
}

///////////////////////////////////////////////////////////

fn run_view_main(state: &mut TerminalState) -> Result<AppState, io::Error> {
    
    match render_main_view(state) {
        UserAction::Select => Ok(AppState::ViewTask),
        UserAction::Quit => Ok(AppState::Done),
        UserAction::Back => Ok(AppState::Done), // back from main => quit
        _ => Ok(AppState::MainMenu) // stay
    }
}

fn run_view_task(state: &mut TerminalState) -> Result<AppState, io::Error> {
    
    let input = 1;
    match input {
        1 => Ok(AppState::MainMenu),
        2 => Ok(AppState::Done),
        _ => Ok(AppState::ViewTask),
    }
}

