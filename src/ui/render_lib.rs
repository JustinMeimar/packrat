use std::fmt;
use std::thread;
use std::time::Duration;
use std::io::{self, Write, Stdout};
use once_cell::sync::Lazy;
use std::sync::Mutex;
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
use crate::ui::render::UserAction;
//
// ///////////////////////////////////////////////////////////
//
// // pub fn render_view<S>(view: S) -> std::io::Result<(Transition)> {
// //
// //     // flush stdout
// //     let mut stdout = std::io::stdout();
// //     execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
// //
// //     // intit the terminal
// //     let backend = CrosstermBackend::new(stdout);
// //     let mut terminal = Terminal::new(backend)?;
// //     crossterm::terminal::enable_raw_mode()?;
// //     
// //     let mut terminal = init_terminal();
// //     let mut action = UserAction::Back;
// //     
// //     // match view {
// //     //     Some(MainView())
// //     // }
// //     loop {
// //         draw_fn(&mut terminal, state);  
// //         action = control_fn(state);
// //         match action {
// //             Transition::Push(_) | Transition::Pop() | Transition::Quit => break,
// //             // UserAction::Select(_) | UserAction::Back | UserAction::Quit => break,
// //             _ => continue
// //         };
// //     }
// //     
// //     // cleanup
// //     crossterm::terminal::disable_raw_mode()?;
// //     execute!(terminal.backend_mut(), crossterm::terminal::LeaveAlternateScreen)?;
// //     terminal.show_cursor()?;
// //
// //     Ok(action)
// // }
//
//
// ///////////////////////////////////////////////////////////
//
// pub fn render_main_view(
//     view: View,
//     state: &mut MainViewState
// ) -> Transition {
//     // pub fn render_main_view(state: &mut TerminalState) -> UserAction { 
//     
//     // set the number of selection options for main view
//     // state. = state.db.get_tasks().len();
//     
//     // let max_idx = db.get_tasks.len();
//
//     // render indefinitely
//     loop {
//         if let Ok(action) = _render_view(
//             state,
//             draw_main_view,
//             control_handler_main
//         ) {
//             return action;
//         }
//     }
// }
//
// pub fn render_task_view(state: &mut TerminalState) -> UserAction {
//     loop {
//         if let Ok(action) = _render_view(
//             state,
//             draw_task_view,
//             control_handler_task
//         ) {
//             return action;
//         }
//     }      
// }
//
// ///////////////////////////////////////////////////////////
//
// fn _render_view<F, G>(
//     state: &mut TerminalState,
//     mut draw_fn: F,
//     mut control_fn: G
// ) -> Result<Transition, std::io::Error>
//
// where F: FnMut(
//             &mut Terminal<CrosstermBackend<Stdout>>,
//             &mut TerminalState
//         ),
//       G: FnMut(
//             &mut TerminalState
//         ) -> UserAction
// { 
//     // flush stdout
//     let mut stdout = std::io::stdout();
//     execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
//
//     // intit the terminal
//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;
//     crossterm::terminal::enable_raw_mode()?;
//     
//     let mut action = UserAction::Back;
//
//     loop {
//         draw_fn(&mut terminal, state);  
//         action = control_fn(state);
//         match action {
//             Transition::Push(_) | Transition::Pop() | Transition::Quit => break,
//             // UserAction::Select(_) | UserAction::Back | UserAction::Quit => break,
//             _ => continue
//         };
//     }
//     
//     // cleanup
//     crossterm::terminal::disable_raw_mode()?;
//     execute!(terminal.backend_mut(), crossterm::terminal::LeaveAlternateScreen)?;
//     terminal.show_cursor()?;
//
//     Ok(action)
// }
//
// fn _sleep(time: u64) {
//     thread::sleep(Duration::from_secs(time));
// }
//
// //////////////////////////////////////////////////////////////////////////////////
//
// fn draw_main_view(
//     terminal: &mut Terminal<CrosstermBackend<Stdout>>,
//     state: &mut TerminalState,
// ) {
//     let widgets = vec![
//         term_user_action_list(),
//         term_user_task_list(state),
//     ];
//     terminal
//         .draw(|f| {
//             let chunks = term_default_layout().split(f.size());
//             widgets.iter().enumerate().for_each(|(i, w)| {
//                 f.render_widget(w.clone(), chunks[i]);
//             });
//         })
//         .unwrap();
// }
//
// fn draw_task_view(
//     terminal: &mut Terminal<CrosstermBackend<Stdout>>,
//     state: &mut TerminalState,
// ) {
//     let widgets = vec![
//         term_user_action_list(),
//         term_task_entries_list(state),
//     ];
//     terminal
//         .draw(|f| {
//             let chunks = term_default_layout().split(f.size());
//             widgets.iter().enumerate().for_each(|(i, w)| {
//                 f.render_widget(w.clone(), chunks[i]);
//             });
//         })
//         .unwrap();
// }
//
// fn control_handler_main(state: &mut TerminalState) -> UserAction {
//     
//     match event::read().unwrap() {  
//         
//         Event::Key(KeyEvent { code: KeyCode::Char('q') | KeyCode::Char('e'), .. }) 
//             => UserAction::Quit, 
//         
//         Event::Key(KeyEvent { code: KeyCode::Char('s') | KeyCode::Enter, .. })
//             => {
//                 let tasks = state.db.get_tasks();
//                 let selected_task = tasks[state.select.idx].clone(); 
//                 UserAction::Select(SelectedItem::Task(selected_task))
//             } 
//         
//         Event::Key(KeyEvent { code: KeyCode::Char('j') | KeyCode::Down, .. })
//             => { 
//                 state.select.decr();
//                 UserAction::None
//             }, 
//         
//         Event::Key(KeyEvent { code: KeyCode::Char('k') | KeyCode::Up, .. })
//             => { 
//                 state.select.incr();
//                 UserAction::None
//             },
//        
//         Event::Key(KeyEvent { code: KeyCode::Char('b'), .. })
//             =>  UserAction::Back,
//         _ 
//             => UserAction::None,
//     }
// }
//
// fn control_handler_task(state: &mut TerminalState) -> UserAction {
//     
//     match event::read().unwrap() {  
//         
//         Event::Key(KeyEvent { code: KeyCode::Char('q') | KeyCode::Char('e'), .. }) 
//             => UserAction::Quit, 
//         
//         Event::Key(KeyEvent { code: KeyCode::Char('s') | KeyCode::Enter, .. })
//             => {
//                 let task = match state.select.item.as_ref() {
//                     Some(SelectedItem::Task(t)) => t,   
//                     _ => panic!("No item is currently selected!")
//                 };
//
//                 let task_entries = state.db.get_task_entries(task.id);
//                 let selected_entry = task_entries[state.select.idx].clone();
//
//                 UserAction::Select(SelectedItem::TaskEntry(selected_entry))
//             } 
//         
//         Event::Key(KeyEvent { code: KeyCode::Char('j') | KeyCode::Down, .. })
//             => { 
//                 state.select.decr();
//                 UserAction::None
//             }, 
//         
//         Event::Key(KeyEvent { code: KeyCode::Char('k') | KeyCode::Up, .. })
//             => { 
//                 state.select.incr();
//                 UserAction::None
//             },
//        
//         Event::Key(KeyEvent { code: KeyCode::Char('b'), .. })
//             =>  UserAction::Back,
//         _ 
//             => UserAction::None,
//     }
// }
//
//
///
///
pub fn term_default_layout() -> Layout {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
}
//
///
///
pub fn term_user_action_list() -> List<'static> {

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
//
// ///
// /// 
// fn term_user_task_list(state: &mut TerminalState) -> List<'static> {
//     
//     let task_list: Vec<ListItem> = state.db.get_tasks()
//         .iter()
//         .enumerate()
//         .map(|(i, task)| style_list_item(&task.name, state.select.idx, i))
//         .collect();
//
//
//     List::new(task_list)
//         .block(Block::default().title("Tasks").borders(Borders::ALL))
//         .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
// }
//
// ///
// /// 
// fn term_task_entries_list(state: &mut TerminalState) -> List<'static> {
//     
//     let task = match state.select.item.as_ref() {
//         Some(SelectedItem::Task(t)) => t,   
//         _ => panic!("No item is currently selected!")
//     };
//
//     let task_list: Vec<ListItem> = state.db.get_task_entries(task.id)
//         .iter()
//         .enumerate()
//         .map(|(i, entry)| style_list_item(&entry.id.to_string(), state.select.idx, i))
//         .collect();
//
//     List::new(task_list)
//         .block(Block::default().title("Task Entries!").borders(Borders::ALL))
//         .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
// }
//
// ///
//
// fn style_list_item(
//     item_text: &str, // Accept a string slice
//     selection_idx: usize,
//     map_idx: usize,
// ) -> ListItem<'static> {
//     let style = if selection_idx == map_idx {
//         Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
//     } else {
//         Style::default()
//     };
//     ListItem::new(Spans::from(Span::styled(item_text.to_string(), style)))
// }
//
//
// fn init_terminal() -> Terminal<CrosstermBackend<Stdout>> {
//     
//     // flush stdout
//     let mut stdout = std::io::stdout();
//     execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
//
//     // intit the terminal
//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;
//     crossterm::terminal::enable_raw_mode()?;
//
//     terminal
// }
//
