use crate::{model::convert::Storable, ui::control::UserAction};
use std::fmt::Display;
use crate::ui::render::renderable::AnyWidget;
use tui::{ 
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Table, Row, Cell, Paragraph},
};


////////////////////////////////////////////////////////////

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
pub fn table_factory<'a, T, K>(
    grid_items: Vec<Vec<T>>,
    table_title: K,
) -> AnyWidget<'a>
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
            .widths(&[Constraint::Min(100)])
    )
}

pub fn paragraph_factory<'a, T>(title: T, init_text: T, highlight: bool) -> AnyWidget<'a>
where
    T: Into<String>
{
    AnyWidget::Paragraph(
        Paragraph::new(init_text.into())
            .block(
                Block::default()
                    .title(title.into())
                    .borders(Borders::ALL)
                    .style(if highlight {
                        Style::default().fg(Color::Yellow)
                    } else {
                        Style::default()
                    }),
            )
    )
}

///////////////////////////////////////////////////////////

pub fn control_widget<'a>() -> AnyWidget<'a> {
    // Collect actions into a Vec<String>
    let actions: Vec<String> = UserAction::all()
        .iter()
        .map(|action| action.to_string())
        .collect();

    let control_string = actions.join(" | ");
    let grid_items = vec![vec![control_string]];

    table_factory(grid_items, "Controls")
}

pub fn map_list_styles<T>(items: &Vec<T>, select_idx: usize) -> Vec<Style>
where
    T: Storable
{ 
    items.iter().enumerate().map(|(i, t)| {
        if i == select_idx {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        }
    }).collect()
}

