use std::io;
use std::io::Stdout;
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
use crate::ui::state::{TaskViewState, MainViewState, EntryViewState};
use crate::{model::task::TaskManager, ui::view::{Transition, View}};

pub trait Controlable {
    
    ///
    fn control(&mut self) -> Transition;
}

impl Controlable for MainViewState {
    
    fn control(&mut self) -> Transition {
        
        match event::read().unwrap() {  

            Event::Key(KeyEvent { code: KeyCode::Char('q') | KeyCode::Char('e'), .. }) 
                => Transition::Quit, 
            
            Event::Key(KeyEvent { code: KeyCode::Char('s') | KeyCode::Enter, .. })
                => {
                    let item = self.items[self.selector.idx].clone();
                    Transition::Push(View::TaskView(TaskViewState::new(item)))
                } 
            
            Event::Key(KeyEvent { code: KeyCode::Char('j') | KeyCode::Down, .. })
                => { 
                    self.selector.decr();
                    Transition::Stay
                }, 
            
            Event::Key(KeyEvent { code: KeyCode::Char('k') | KeyCode::Up, .. })
                => { 
                    self.selector.incr();
                    Transition::Stay
                },
           
            Event::Key(KeyEvent { code: KeyCode::Char('b'), .. })
                =>  Transition::Stay,
            _ 
                => Transition::Stay,
        } 
    }
}

impl Controlable for TaskViewState {
    
    fn control(&mut self) -> Transition {
        
        match event::read().unwrap() {  

            Event::Key(KeyEvent { code: KeyCode::Char('q') | KeyCode::Char('e'), .. }) 
                => Transition::Quit, 
            
            Event::Key(KeyEvent { code: KeyCode::Char('s') | KeyCode::Enter, .. })
                => {
                    let item = self.items[self.selector.idx].clone();
                    Transition::Push(View::EntryView(EntryViewState::new(item)))
                } 
            
            Event::Key(KeyEvent { code: KeyCode::Char('j') | KeyCode::Down, .. })
                => { 
                    self.selector.decr();
                    Transition::Stay
                }, 
            
            Event::Key(KeyEvent { code: KeyCode::Char('k') | KeyCode::Up, .. })
                => { 
                    self.selector.incr();
                    Transition::Stay
                },
           
            Event::Key(KeyEvent { code: KeyCode::Char('b'), .. })
                =>  Transition::Pop,
            _ 
                => Transition::Stay,
        } 
    }
}

