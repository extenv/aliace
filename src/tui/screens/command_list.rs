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

    let filtered_commands = app.get_filtered_commands();
    let filtered_groups = app.get_filtered_groups();
    let filtered_favorites = app.get_filtered_favorites();

    let is_empty = match app.list_tab {
        0 => filtered_commands.is_empty(),
        1 => filtered_groups.is_empty(),
        _ => filtered_favorites.is_empty(),
    };
    
    let max_len = match app.list_tab {
        0 => filtered_commands.len(),
        1 => filtered_groups.len(),
        _ => filtered_favorites.len(),
    };

    if max_len == 0 {
        app.list_selected = 0;
    } else if app.list_selected >= max_len {
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
            Constraint::Length(3), // Search Bar
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
    let tab2_style = if app.list_tab == 2 {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let tab_line = Line::from(vec![
        Span::styled(if app.list_tab == 0 { " ● Single " } else { " ○ Single " }, tab0_style),
        Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
        Span::styled(if app.list_tab == 1 { " ● Group " } else { " ○ Group " }, tab1_style),
        Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
        Span::styled(if app.list_tab == 2 { " ● Favorites " } else { " ○ Favorites " }, tab2_style),
    ]);
    frame.render_widget(Paragraph::new(tab_line), list_layout[0]);

    let divider = Paragraph::new(Span::styled("─".repeat(list_layout[1].width as usize), Style::default().fg(Color::DarkGray)));
    frame.render_widget(divider, list_layout[1]);

    // Render Search Bar
    let search_style = if app.list_is_searching {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    
    let search_title = if app.list_is_searching {
        " 🔍 Search (Type query, Enter to Run, Esc to unfocus) "
    } else {
        " 🔍 Search (Press [/] to focus) "
    };
    
    let search_block = Block::default()
        .title(search_title)
        .borders(Borders::ALL)
        .border_style(search_style);
        
    let search_text = if app.list_is_searching {
        format!("{}█", app.list_search_query)
    } else if app.list_search_query.is_empty() {
        " (press / to start searching...)".to_string()
    } else {
        app.list_search_query.clone()
    };
    
    let search_para = Paragraph::new(search_text)
        .block(search_block)
        .style(if app.list_search_query.is_empty() && !app.list_is_searching {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default().fg(Color::White)
        });
    frame.render_widget(search_para, list_layout[2]);

    let items: Vec<ListItem> = match app.list_tab {
        0 => {
            filtered_commands.iter().enumerate().map(|(i, cmd)| {
                let is_grabbed = app.list_grabbed == Some(i);
                let is_fav = cmd.favorite;
                let fav_star = if is_fav { " ★" } else { "" };
                let style = if i == app.list_selected {
                    if is_grabbed {
                        Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
                    }
                } else {
                    if is_grabbed {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    }
                };
                let prefix = if is_grabbed { " 🚀 " } else { "  " };
                ListItem::new(format!("{}{}{}  ", prefix, cmd.title, fav_star)).style(style)
            }).collect()
        }
        1 => {
            filtered_groups.iter().enumerate().map(|(i, grp)| {
                let is_grabbed = app.list_grabbed == Some(i);
                let is_fav = grp.favorite;
                let fav_star = if is_fav { " ★" } else { "" };
                let style = if i == app.list_selected {
                    if is_grabbed {
                        Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
                    }
                } else {
                    if is_grabbed {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    }
                };
                let prefix = if is_grabbed { " 🚀 " } else { "  " };
                ListItem::new(format!("{}{}{}  ", prefix, grp.name, fav_star)).style(style)
            }).collect()
        }
        _ => {
            filtered_favorites.iter().enumerate().map(|(i, item)| {
                let is_grabbed = app.list_grabbed == Some(i);
                let style = if i == app.list_selected {
                    if is_grabbed {
                        Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
                    }
                } else {
                    if is_grabbed {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    }
                };
                let prefix = if is_grabbed { " 🚀 " } else { "  " };
                let type_tag = if item.is_group { " [G]" } else { " [S]" };
                ListItem::new(format!("{}{:<20} ({} runs){} ★  ", prefix, item.name, item.use_count, type_tag)).style(style)
            }).collect()
        }
    };

    let list_widget = List::new(items)
        .block(Block::default().borders(Borders::NONE))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_widget(list_widget, list_layout[3]);

    let details_block = Block::default()
        .title(" Details ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Gray));
    
    let inner_details_area = details_block.inner(chunks[1]);
    frame.render_widget(details_block, chunks[1]);

    if is_empty {
        let msg = match app.list_tab {
            0 => "\n  No single commands match search query or exist. Press [A] to add a command.",
            1 => "\n  No group sequences match search query or exist. Press [G] to add a group sequence.",
            _ => "\n  No commands or group sequences match search query or have been run.",
        };
        let paragraph = Paragraph::new(msg).wrap(Wrap { trim: true });
        frame.render_widget(paragraph, inner_details_area);
    } else {
        if app.list_tab == 0 {
            let cmd = &filtered_commands[app.list_selected];
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
        } else if app.list_tab == 1 {
            let grp = &filtered_groups[app.list_selected];
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
        } else {
            // Favorites tab selected item details
            let item = &filtered_favorites[app.list_selected];
            if !item.is_group {
                if let Some(cmd) = app.db.commands.iter().find(|c| c.title == item.name) {
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
                }
            } else {
                if let Some(grp) = app.db.groups.iter().find(|g| g.name == item.name) {
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
    }
}
