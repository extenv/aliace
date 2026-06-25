use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn draw_export(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" Export Backup Config ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Gray));
    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(6),
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(inner_area);

    let items = vec![
        ListItem::new(if app.export_selected == 0 { "  [x] Commands database only " } else { "  [ ] Commands database only " }),
        ListItem::new(if app.export_selected == 1 { "  [x] Groups database only " } else { "  [ ] Groups database only " }),
        ListItem::new(if app.export_selected == 2 { "  [x] Everything (Commands, Groups & History) " } else { "  [ ] Everything (Commands, Groups & History) " }),
    ];

    let list = List::new(items)
        .block(Block::default().title(" Select Export Type ").borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)));
    frame.render_widget(list, chunks[1]);

    if let Some(msg) = &app.export_message {
        let para = Paragraph::new(format!("  {}", msg))
            .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD));
        frame.render_widget(para, chunks[3]);
    }
}

pub fn draw_import(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" Import Backup JSON ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Gray));
    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(inner_area);

    let path_block = Block::default()
        .title(" JSON File Absolute Path ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    let path_para = Paragraph::new(format!(" {}", app.import_path)).block(path_block);
    frame.render_widget(path_para, chunks[1]);

    if let Some(msg) = &app.import_message {
        let color = if msg.contains("Error") { Color::Red } else { Color::Green };
        let para = Paragraph::new(format!("  {}", msg))
            .style(Style::default().fg(color).add_modifier(Modifier::BOLD));
        frame.render_widget(para, chunks[3]);
    }
}
