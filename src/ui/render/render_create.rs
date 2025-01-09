use std::io;
use crate::model::convert::Storable;
use crate::ui::view::Transition;
use crate::ui::state::CreateViewState;
use crate::ui::control::Controlable;
use tui::{ 
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};
use crate::ui::render::renderable::{
    Renderable,
    render_view_startup, render_view_teardown
};

///////////////////////////////////////////////////////////

impl<T: Storable> Renderable for CreateViewState<T> {

    /// TODO: This implementation is fully rolled out for first iteration, needs
    /// to be factored into the widget library
    fn render(&mut self) -> io::Result<Transition> {
        
        let mut terminal = render_view_startup()?;
        let mut transition = Transition::Stay; 

        let result = loop {
            terminal.draw(|f| {
                let size = f.size();
                let modal_width = size.width / 3 * 2;
                let modal_height = 10;

                let modal_area = tui::layout::Rect::new(
                    (size.width - modal_width) / 2,
                    (size.height - modal_height) / 2,
                    modal_width,
                    modal_height,
                );
                
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ])
                    .split(modal_area);

                let modal_block = Block::default()
                    .title("Create New Task")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Gray));
                f.render_widget(modal_block, modal_area);

                let title_widget = Paragraph::new(self.inputs[0].as_str())
                    .block(
                        Block::default()
                            .title("Task Title")
                            .borders(Borders::ALL)
                            .style(if self.active_input == 0 {
                                Style::default().fg(Color::Yellow)
                            } else {
                                Style::default()
                            }),
                    );

                let desc_widget = Paragraph::new(self.inputs[1].as_str())
                    .block(
                        Block::default()
                            .title("Task Description")
                            .borders(Borders::ALL)
                            .style(if self.active_input == 1 {
                                Style::default().fg(Color::Yellow)
                            } else {
                                Style::default()
                            }),
                    );

                f.render_widget(title_widget, chunks[0]);
                f.render_widget(desc_widget, chunks[1]);
            })?;
            
            transition = self.control();
            match transition {
                Transition::Stay => continue,
                _ => break
            } 
        };

        // Ensure the terminal is properly torn down before returning
        render_view_teardown(&mut terminal)?;
        Ok(transition)
    }
}

