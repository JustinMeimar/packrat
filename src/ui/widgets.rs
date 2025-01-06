use crate::ui::control::UserAction;

use tui::{ 
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
};

pub fn term_default_layout() -> Layout {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(15), Constraint::Percentage(85)].as_ref())
}

pub fn term_user_action_list() -> List<'static> {

    let items: Vec<ListItem> = UserAction::all()
        .iter()
        .map(|x| ListItem::new(format!("{}", x)))
        .collect();
    
    List::new(items)
        .block(Block::default().title("Controls").borders(Borders::ALL))
        .style(Style::default().fg(Color::Gray))
}

