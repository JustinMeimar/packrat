use crate::ui::control::UserAction;
use tui::{ 
    text::{Span, Spans},
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Table, Widget, Row, Cell},
    Terminal,
};

use std::fmt::Display;
use crate::ui::render::AnyWidget;

pub fn list_factory<'a, T, K>(
    list_items: Vec<T>,
    list_styles: Vec<Style>,
    list_title: K
) -> AnyWidget<'a>
where
    T: Display,
    K: Into<String>,
{    
    // Create a default list widget
    let items: Vec<ListItem> = list_items
        .iter()
        .enumerate()
        .map(|(i, x)| ListItem::new(format!("{}", x)).style(list_styles[i]))
        .collect();
 
    AnyWidget::List(
        List::new(items)
            .block(Block::default().title(list_title.into()).borders(Borders::ALL))
            .style(Style::default().fg(Color::Gray)),
    )
}

/// Create a table widget from a vector of vectors. The inner vector represents a row,
/// by convention the first row is used as the column labels.
pub fn table_factory<'a, T, K>(grid_items: Vec<Vec<T>>, table_title: K) -> AnyWidget<'a>
where
    T: Display,
    K: Into<String>,
{
    let table_rows: Vec<Row<'a>> = grid_items
        .iter()
        .skip(1)
        .map(|row_items| {
            Row::new(
                row_items
                    .iter()
                    .map(|item| Cell::from(item.to_string()))
                    .collect::<Vec<Cell>>(),
            )
        })
        .collect();

    let column_names: Vec<String> = grid_items
        .first()
        .unwrap()
        .iter()
        .map(|x| x.to_string())
        .collect();

    AnyWidget::Table(
        Table::new(table_rows)
            .header(Row::new(column_names))
            .block(Block::default().title(table_title.into()).borders(Borders::ALL))
            .widths(&[Constraint::Min(10); 3]),
    )
}

///////////////////////////////////////////////////////////

// pub fn term_default_layout() -> Layout {
//     Layout::default()
//         .direction(Direction::Vertical)
//         .constraints([Constraint::Percentage(15), Constraint::Percentage(85)].as_ref())
// }
//
// pub fn term_user_action_list() -> List<'static> {
//
//     let items: Vec<ListItem> = UserAction::all()
//         .iter()
//         .map(|x| ListItem::new(format!("{}", x)))
//         .collect();
//     
//     List::new(items)
//         .block(Block::default().title("Controls").borders(Borders::ALL))
//         .style(Style::default().fg(Color::Gray))
// }

