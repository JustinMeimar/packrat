use std::io;

#[derive(Clone, Debug)]
enum UserAction {
    Select,
    Back,
    Quit
}

impl UserAction {
    pub fn all() -> Vec<UserAction> {
        vec![
            UserAction::Select,
            UserAction::Back,
            UserAction::Quit
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

struct TerminalState { 
    select_idx: usize
}

impl TerminalState {
    pub fn new() -> Self { TerminalState { select_idx: 0 }}
}


fn run_view_main(state: &mut TerminalState) -> Result<AppState, io::Error> {
    
    let input = 1;
    match input {
        1 => Ok(AppState::MainMenu),
        2 => Ok(AppState::Done),
        _ => Ok(AppState::ViewTask),
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


pub fn start() -> Result<(), io::Error> {
    
    let mut app_state = AppState::MainMenu;
    let mut term_state = TerminalState::new();

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

