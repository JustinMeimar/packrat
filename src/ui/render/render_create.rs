use std::io;
use crate::model::{convert::Storable, task_entry::TaskEntry, task::Task};
use crate::model::store::TaskStore;
use crate::ui::view::Transition;
use crate::ui::state::{CreateEntryViewState, CreateTaskViewState};
use crate::ui::control::Controlable;
use tui::{ 
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};
use crate::ui::render::renderable::{
    Renderable, AnyWidget, render_view,
    render_view_startup, render_view_teardown
};
use crate::ui::widgets::paragraph_factory;
use crossterm::event::{self, Event, KeyCode, KeyEvent};

///////////////////////////////////////////////////////////

/// Similar to Renderable for Regular views, but with validation,
/// and tighter abstractions
pub trait FormRenderable {
    
    ///
    fn chunks(&self, frame: Rect) -> Vec<Rect> {
        
        let modal_width = frame.width / 3 * 2;
        let modal_height = 10; 
        let modal_area = tui::layout::Rect::new(
                    (frame.width - modal_width) / 2,
                    (frame.height - modal_height) / 2,
                    modal_width,
                    modal_height,
                );

        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(modal_area)
    } 
    
    /// Valid by default 
    fn validate() -> bool { true } 
    
    /// Shared method with Renderable trait
    fn render(&mut self) -> io::Result<Transition> {
        Ok(Transition::Stay) 
    }
    
    /// Must implement!
    fn controller(&mut self) -> Transition;

    /// Must implement!
    fn widgets(&mut self) -> io::Result<Vec<AnyWidget>>;

}


impl FormRenderable for CreateTaskViewState {
    
    ///
    fn widgets(&mut self) -> io::Result<Vec<AnyWidget>> {
        
        let title_widget = paragraph_factory(
            "Task Name", self.inputs[0].as_str(), (self.active_input == 0));
        
        let desc_widget = paragraph_factory(
            " Task Descripion", self.inputs[1].as_str(), (self.active_input == 1));
        
        Ok(vec![title_widget, desc_widget])
    }

    /// Render a dialgoue box overtop of the current view, taking the user input.
    fn render(&mut self) -> io::Result<Transition> {
        
        let mut terminal = render_view_startup()?;
        let mut transition = Transition::Stay; 

        let result = loop {
            terminal.draw(|f| {
                
                let chunks: Vec<Rect> = self.chunks(f.size()); 
                let widgets = self.widgets().unwrap();
                
                widgets.into_iter().enumerate().for_each(|(i, w)| {
                    f.render_widget(w, chunks[i]);
                });
 
            })?;
            
            transition = self.controller(); 
            match transition {
                Transition::Stay => continue,
                _ => break
            } 
        };

        // Ensure the terminal is properly torn down before returning
        render_view_teardown(&mut terminal)?;
        Ok(transition)
    }

    fn controller(&mut self) -> Transition {
        match event::read().unwrap() {
            Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => Transition::Pop,
            Event::Key(KeyEvent { code: KeyCode::Char(c), .. }) => {
                self.inputs[self.active_input].push(c);
                Transition::Stay
            }
            Event::Key(KeyEvent { code: KeyCode::Backspace, .. }) => {
                self.inputs[self.active_input].pop();
                Transition::Stay
            }
            Event::Key(KeyEvent { code: KeyCode::Tab, .. }) => {
                let n_input = self.inputs.len();
                self.active_input = (self.active_input + n_input + 1) % n_input;
                Transition::Stay
            }
            Event::Key(KeyEvent { code: KeyCode::Enter, .. }) => {
                TaskStore::instance().put(
                    Task::new(
                        self.inputs[0].clone(),
                        self.inputs[1].clone())
                    );
                Transition::Pop
            }
            _ => Transition::Stay,
        }
    }
}

impl FormRenderable for CreateEntryViewState {
    
    fn widgets(&mut self) -> io::Result<Vec<AnyWidget>> {
        
        let title_widget = paragraph_factory(
            "Entry Name", self.inputs[0].as_str(), (self.active_input == 0));
        
        let desc_widget = paragraph_factory(
            "Entry Date", self.inputs[1].as_str(), (self.active_input == 1));
        
        Ok(vec![title_widget, desc_widget])
    }

    /// Render a dialgoue box overtop of the current view, taking the user input.
    fn render(&mut self) -> io::Result<Transition> {
        
        let mut terminal = render_view_startup()?;
        let mut transition = Transition::Stay; 

        let result = loop {
            terminal.draw(|f| {
                
                let chunks: Vec<Rect> = self.chunks(f.size()); 
                let widgets = self.widgets().unwrap();
                
                widgets.into_iter().enumerate().for_each(|(i, w)| {
                    f.render_widget(w, chunks[i]);
                });
 
            })?;
            
            transition = self.controller(); 
            match transition {
                Transition::Stay => continue,
                _ => break
            } 
        };

        // Ensure the terminal is properly torn down before returning
        render_view_teardown(&mut terminal)?;
        Ok(transition)
    }

    fn controller(&mut self) -> Transition {
        match event::read().unwrap() {
            Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => Transition::Pop,
            Event::Key(KeyEvent { code: KeyCode::Char(c), .. }) => {
                self.inputs[self.active_input].push(c);
                Transition::Stay
            }
            Event::Key(KeyEvent { code: KeyCode::Backspace, .. }) => {
                self.inputs[self.active_input].pop();
                Transition::Stay
            }
            Event::Key(KeyEvent { code: KeyCode::Tab, .. }) => {
                let n_input = self.inputs.len();
                self.active_input = (self.active_input + n_input + 1) % n_input;
                Transition::Stay
            }
            Event::Key(KeyEvent { code: KeyCode::Enter, .. }) => {
                TaskStore::instance().put(TaskEntry::new(
                        self.item.id, vec![] 
                    ));
                Transition::Pop
            }
            _ => Transition::Stay,
        }
    }
}

