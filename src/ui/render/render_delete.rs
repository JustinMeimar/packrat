use std::io;
use crate::log::debug_log;
use crate::model::{convert::Storable, task_entry::TaskEntry, task::Task};
use crate::model::store::TaskStore;
use crate::ui::view::Transition;
use crate::ui::state::DeleteViewState;
use crate::ui::render::render_create::FormRenderable;
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
use std::fmt::Display;

///////////////////////////////////////////////////////////

impl<T: Display + Storable> Renderable for DeleteViewState<T> {
    
    ///
    fn widgets(&mut self) -> io::Result<Vec<AnyWidget>> {
            
        let delete_text = format!("Delete item: {}? (Y/N)", self.delete_item);

        Ok(vec![
            AnyWidget::Paragraph(
                Paragraph::new(delete_text)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                    )
                    .style(
                        Style::default()

                    )
            )]
        )
    }
    
    /// Center chunk, right in the middle
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

    /// Render a dialgoue box overtop of the current view, taking the user input.
    fn render(&mut self) -> io::Result<Transition> {
        render_view(self, Self::controler)  
    }
    
    /// delete controls
    fn controler(&mut self) -> Transition {
        match event::read().unwrap() {
            
            Event::Key(KeyEvent { code: KeyCode::Char('Y') | KeyCode::Char('y'), .. })
                => {
                    match TaskStore::instance().delete_item(&self.delete_item) {
                        Ok(()) => Transition::Pop,
                        Err(e) => panic!("This is a patrack bug!")
                    }
                }, 
            
            Event::Key(KeyEvent { code: KeyCode::Char('N')
                | KeyCode::Char('n')
                | KeyCode::Esc
                | KeyCode::Char('q'), .. }) => Transition::Pop,
            
            _ => Transition::Stay,
        }
    }
}

