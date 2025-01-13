use std::io;
use crate::model::convert::Storable;
use crate::model::store::TaskStore;
use crate::ui::view::Transition;
use crate::ui::state::PopUpViewState;
use tui::{ 
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};
use crate::ui::render::renderable::{
    Renderable, AnyWidget, render_view
};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::fmt::Display;

///////////////////////////////////////////////////////////

impl Renderable for PopUpViewState {
    
    ///
    fn widgets(&mut self) -> io::Result<Vec<AnyWidget>> {

        Ok(vec![
            AnyWidget::Paragraph(
                Paragraph::new(self.text.clone())
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                    )
                    .style(
                        Style::default().fg(Color::Yellow)
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
            _ => Transition::Pop,
        }
    }
}

