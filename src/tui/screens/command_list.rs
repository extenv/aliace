use crate::app::{App, AppScreen};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn draw_command_list(frame: &mut Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(35),
            Constraint::Percentage(65),
        ])
        .split(area);

    let is_empty = if app.list_tab == 0 {
        app.db.commands.is_empty()
    } else {
        app.db.groups.is_empty()
    };
    
    let max_len = if app.list_tab == 0 {
        app.db.commands.len()
    } else {
        app.db.groups.len()
    };

    if max_len > 0 && app.list_selected >= max_len {
        app.list_selected = max_len - 1;
    }

    let list_title = match app.screen {
        AppScreen::ListCommands => " Select Command / Group (Press Enter to Run) ",
        AppScreen::UpdateCommandList => " Choose Command / Group to Edit ",
        AppScreen::DeleteCommandList => " Choose Command / Group to Delete ",
        _ => " Commands & Groups ",
    };

    let list_block = Block::default()
        .title(list_title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    
    let inner_list_area = list_block.inner(chunks[0]);
    frame.render_widget(list_block, chunks[0]);

    let list_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Tab Bar
            Constraint::Length(1), // Divider line
            Constraint::Min(1),    // Items
        ])
        .split(inner_list_area);

    let tab0_style = if app.list_tab == 0 {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let tab1_style = if app.list_tab == 1 {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let tab_line = Line::from(vec![
        Span::styled(if app.list_tab == 0 { " ● Single " } else { " ○ Single " }, tab0_style),
        Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
        Span::styled(if app.list_tab == 1 { " ● Group " } else { " ○ Group " }, tab1_style),
    ]);
    frame.render_widget(Paragraph::new(tab_line), list_layout[0]);

    let divider = Paragraph::new(Span::styled("─".repeat(list_layout[1].width as usize), Style::default().fg(Color::DarkGray)));
    frame.render_widget(divider, list_layout[1]);

    let items: Vec<ListItem> = if app.list_tab == 0 {
        app.db.commands.iter().enumerate().map(|(i, cmd)| {
            let style = if i == app.list_selected {
                Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(format!("  {}  ", cmd.title)).style(style)
        }).collect()
    } else {
        app.db.groups.iter().enumerate().map(|(i, grp)| {
            let style = if i == app.list_selected {
                Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(format!("  {}  ", grp.name)).style(style)
        }).collect()
    };

    let list_widget = List::new(items)
        .block(Block::default().borders(Borders::NONE))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_widget(list_widget, list_layout[2]);

    let details_block = Block::default()
        .title(" Details ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Gray));
    
    let inner_details_area = details_block.inner(chunks[1]);
    frame.render_widget(details_block, chunks[1]);

    if is_empty {
        let msg = if app.list_tab == 0 {
            "\n  No single commands available. Press [A] to add a command."
        } else {
            "\n  No group sequences available. Press [G] to add a group sequence."
        };
        let paragraph = Paragraph::new(msg).wrap(Wrap { trim: true });
        frame.render_widget(paragraph, inner_details_area);
    } else {
        if app.list_tab == 0 {
            let cmd = &app.db.commands[app.list_selected];
            let detail_text = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled("  TYPE:        ", Style::default().fg(Color::Gray)),
                    Span::styled("SINGLE COMMAND", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(vec![
                    Span::styled("  TITLE:       ", Style::default().fg(Color::Gray)),
                    Span::styled(&cmd.title, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(vec![
                    Span::styled("  DESCRIPTION: ", Style::default().fg(Color::Gray)),
                    Span::styled(&cmd.description, Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("  USE COUNT:   ", Style::default().fg(Color::Gray)),
                    Span::styled(format!("{} times", cmd.use_count), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  COMMAND SCRIPT:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(vec![
                    Span::styled(format!("    {}", cmd.script), Style::default().fg(Color::Cyan)),
                ]),
            ];
            let paragraph = Paragraph::new(detail_text).wrap(Wrap { trim: true });
            frame.render_widget(paragraph, inner_details_area);
        } else {
            let grp = &app.db.groups[app.list_selected];
            let mut detail_text = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled("  TYPE:        ", Style::default().fg(Color::Gray)),
                    Span::styled("GROUP COMMAND (MULTI-EXECUTE)", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(vec![
                    Span::styled("  GROUP NAME:  ", Style::default().fg(Color::Gray)),
                    Span::styled(&grp.name, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(vec![
                    Span::styled("  DESCRIPTION: ", Style::default().fg(Color::Gray)),
                    Span::styled(&grp.description, Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("  USE COUNT:   ", Style::default().fg(Color::Gray)),
                    Span::styled(format!("{} times", grp.use_count), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  EXECUTION SEQUENCE:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                ]),
            ];
            
            if grp.commands.is_empty() {
                detail_text.push(Line::from("    (No commands in group sequence)"));
            } else {
                for (i, cmd_title) in grp.commands.iter().enumerate() {
                    let script = app.db.commands.iter()
                        .find(|c| &c.title == cmd_title)
                        .map(|c| c.script.as_str())
                        .unwrap_or("(Command not found)");
                    detail_text.push(Line::from(vec![
                        Span::styled(format!("    {}. {:<15} ", i + 1, cmd_title), Style::default().fg(Color::White)),
                        Span::styled(format!("-> {}", script), Style::default().fg(Color::Cyan)),
                    ]));
                }
            }
            
            let paragraph = Paragraph::new(detail_text).wrap(Wrap { trim: true });
            frame.render_widget(paragraph, inner_details_area);
        }
    }
}
