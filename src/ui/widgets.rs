use crate::{model::convert::Storable, ui::control::UserAction};
use std::fmt::Display;
use crate::ui::render::renderable::AnyWidget;
use crate::model::task::Task;
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
        .map(|(i, x)| ListItem::new(format!(" {}", x)).style(list_styles[i]))
        .collect();
 
    AnyWidget::List(
        List::new(items)
            .block(Block::default().title(list_title.into()).borders(Borders::ALL))
            .style(Style::default().fg(Color::Gray)),
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
    
    let control_string = UserAction::all()
        .iter()
        .map(|action| action.to_string())
        .collect::<Vec<_>>()
        .join(" | ");

    let control_row = Row::new(vec![Cell::from(control_string)]);
    let table = Table::new(vec![control_row])
        .block(
            Block::default()
                .title("Controls")
                .borders(Borders::ALL),
        )
        .widths(&[Constraint::Percentage(100)])
        .column_spacing(1);

    AnyWidget::Table(table)
}

// pub fn item_table<'a, T>(
//     mut tasks: &Vec<T>,
//     column_headers: Vec<&str>,
//     constraints: Vec<Constraint>,
//     select_idx: usize
// ) -> AnyWidget<'a>
//
// where
//     T: Storable
// {  
//     // create and style the rows
//     let style = Style::default();
//     let task_rows: Vec<Row> = tasks
//         .iter()
//         .enumerate()
//         .map(|(i, t)| Row::new(t.get_display_fields())
//             .style(if i == select_idx {
//                 style.fg(Color::Yellow).add_modifier(Modifier::BOLD)
//             } else {
//                 style
//             }))
//         .collect();
//     
//     let column_labels: Vec<Cell> = column_headers 
//         .iter()
//         .map(|s| Cell::from(s.to_string()))
//         .collect();
//     
//     let table = Table::new(task_rows)
//         .block(Block::default().title("Tasks").borders(Borders::ALL))
//         .header(Row::new(column_labels))
//         .widths(&constraints) 
//         .column_spacing(2);
//     
//     AnyWidget::Table(table) 
// }

pub fn item_table<'a, T>(
    tasks: &'a [T],
    column_headers: &'a [&'a str],
    constraints: &'a [Constraint],
    select_idx: usize,
) -> AnyWidget<'a>
where
    T: Storable,
{
    let style = Style::default();

    let task_rows: Vec<Row> = tasks
        .iter()
        .enumerate()
        .map(|(i, t)| {
            Row::new(t.get_display_fields()).style(
                if i == select_idx {
                    style.fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    style
                },
            )
        })
        .collect();

    let column_labels: Vec<Cell> = column_headers
        .iter()
        .map(|s| Cell::from(s.to_string()))
        .collect();

    let table = Table::new(task_rows)
        .block(Block::default().title("Tasks").borders(Borders::ALL))
        .header(Row::new(column_labels))
        .widths(constraints) // references constraints up in the caller
        .column_spacing(2);

    AnyWidget::Table(table)
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

