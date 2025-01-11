/// control.rs

use std::fmt;
use std::collections::HashMap;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crate::model::convert::Storable;
use crate::model::task::Task;
use crate::model::store::TaskStore;
use crate::ui::view::Transition;

///////////////////////////////////////////////////////////

type KeyHandler = HashMap<KeyCode, Transition>;

#[derive(Clone, Debug)]
pub enum UserAction {
    Select,
    New,
    Edit,
    Delete,
    Back,
    Quit,
}

impl UserAction {
    pub fn all() -> Vec<UserAction> {
        vec![
            UserAction::Select,
            UserAction::New,
            UserAction::Edit,
            UserAction::Delete,
            UserAction::Back,
            UserAction::Quit,
        ]
    } 

    pub fn from_index(index: usize) -> Self {
        UserAction::all()[index].clone()
    }    
}

impl fmt::Display for UserAction {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            UserAction::Select => "Select (s)",  
            UserAction::Back => "Back (b)",  
            UserAction::New => "New (n)",  
            UserAction::Delete => "Delete (d)",  
            UserAction::Edit => "Edit (e)",  
            UserAction::Quit => "Quit (q)",  
        };
        write!(fmt, "{}", text)
    }
} 

//////////////////////////////////////////////////////////

pub trait Controlable { 
    /// keyboard handler
    fn control(&mut self) -> Transition;    
}

// impl<T: Storable> Controlable for CreateViewState<T> {
//     
//     fn control(&mut self) -> Transition {
//
//
//         match event::read().unwrap() {  
//     
//             Event::Key(KeyEvent { code: KeyCode::Esc, .. })
//                 => Transition::Pop,
//
//             Event::Key(KeyEvent {code: KeyCode::Char(c), .. })
//                 => {
//                     self.inputs[self.active_input].push(c);
//                     Transition::Stay
//                 }
//             
//             Event::Key(KeyEvent {code: KeyCode::Backspace, .. })
//                 => {
//                     self.inputs[self.active_input].pop();
//                     Transition::Stay
//                 }
//  
//             Event::Key(KeyEvent {code: KeyCode::Tab, .. })
//                 => {
//                     let n_input = self.inputs.len();
//                     self.active_input = (self.active_input + n_input + 1) % n_input; 
//                     Transition::Stay
//                 }
//  
//             Event::Key(KeyEvent { code: KeyCode::Enter, .. })
//                 => {
//
//                     TaskStore::instance().put(
//                         Task::new(
//                             self.inputs[0].clone(),
//                             self.inputs[1].clone()
//                         )
//                     );
//                     Transition::Pop
//                 }  
//             _ 
//                 => Transition::Stay,
//         }
//     }
// }

