use crate::app::{App, UsedItem};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn draw_dashboard(frame: &mut Frame, area: Rect, app: &App) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Cards
            Constraint::Min(5),    // Details & logs
        ])
        .split(area);

    // Render Top Cards (4 columns)
    let cards_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
        ])
        .split(main_chunks[0]);

    let total_commands = app.db.commands.len();
    let total_groups = app.db.groups.len();
    let total_runs = app.db.history.len();
    let total_favorites = app.db.commands.iter().filter(|c| c.favorite).count()
        + app.db.groups.iter().filter(|g| g.favorite).count();

    // Card 1: Single Commands
    let card1_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(format!("{}", total_commands), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]).alignment(Alignment::Center),
    ];
    let card1_para = Paragraph::new(card1_text)
        .block(Block::default()
            .title(Span::styled(" Single Commands ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .border_type(BorderType::Rounded)
        );
    frame.render_widget(card1_para, cards_chunks[0]);

    // Card 2: Group Sequences
    let card2_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(format!("{}", total_groups), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]).alignment(Alignment::Center),
    ];
    let card2_para = Paragraph::new(card2_text)
        .block(Block::default()
            .title(Span::styled(" Group Sequences ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .border_type(BorderType::Rounded)
        );
    frame.render_widget(card2_para, cards_chunks[1]);

    // Card 3: Total Executions
    let card3_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(format!("{}", total_runs), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]).alignment(Alignment::Center),
    ];
    let card3_para = Paragraph::new(card3_text)
        .block(Block::default()
            .title(Span::styled(" Total Executions ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .border_type(BorderType::Rounded)
        );
    frame.render_widget(card3_para, cards_chunks[2]);

    // Card 4: Total Favorites
    let card4_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(format!("{}", total_favorites), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]).alignment(Alignment::Center),
    ];
    let card4_para = Paragraph::new(card4_text)
        .block(Block::default()
            .title(Span::styled(" Total Favorites ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .border_type(BorderType::Rounded)
        );
    frame.render_widget(card4_para, cards_chunks[3]);

    // Render Bottom Details
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(60),
        ])
        .split(main_chunks[1]);

    let mut sorted_items = vec![];
    for cmd in &app.db.commands {
        sorted_items.push(UsedItem {
            name: cmd.title.clone(),
            is_group: false,
            use_count: cmd.use_count,
        });
    }
    for grp in &app.db.groups {
        sorted_items.push(UsedItem {
            name: grp.name.clone(),
            is_group: true,
            use_count: grp.use_count,
        });
    }
    sorted_items.sort_by(|a, b| b.use_count.cmp(&a.use_count));
    let limit = bottom_chunks[0].height.saturating_sub(4) as usize;
    let top_items = sorted_items.iter().take(limit);

    let mut list_items = vec![];
    if sorted_items.is_empty() {
        list_items.push(ListItem::new("  No runs logged yet."));
    } else {
        for (i, item) in top_items.enumerate() {
            let type_tag = if item.is_group { "[Group]  " } else { "[Single] " };
            let type_color = if item.is_group { Color::Magenta } else { Color::Green };
            
            let is_fav = if item.is_group {
                app.db.groups.iter().any(|g| g.name == item.name && g.favorite)
            } else {
                app.db.commands.iter().any(|c| c.title == item.name && c.favorite)
            };
            let fav_star = if is_fav { " ★" } else { "  " };
            
            // Align the list items perfectly ("sejajar")
            let display_name = if item.name.len() > 15 {
                format!("{}...", &item.name[..12])
            } else {
                item.name.clone()
            };
            
            list_items.push(ListItem::new(Line::from(vec![
                Span::styled(format!(" {:>2}. ", i + 1), Style::default().fg(Color::White)),
                Span::styled(type_tag, Style::default().fg(type_color)),
                Span::styled(format!("{:<15}", display_name), Style::default().fg(Color::White)),
                Span::styled(fav_star, Style::default().fg(Color::Yellow)),
                Span::styled(format!(" ({} executed)", item.use_count), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            ])));
        }
    }

    let most_run_border_style = if app.dashboard_focused_panel == 0 {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::Gray)
    };

    let stats_widget = List::new(list_items)
        .block(Block::default()
            .title(" 🔥 Most Run Items ")
            .borders(Borders::ALL)
            .border_style(most_run_border_style)
        )
        .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    let mut list_state = ratatui::widgets::ListState::default();
    if !sorted_items.is_empty() && app.dashboard_focused_panel == 0 {
        list_state.select(Some(app.dashboard_selected));
    }
    frame.render_stateful_widget(stats_widget, bottom_chunks[0], &mut list_state);

    let history_border_style = if app.dashboard_focused_panel == 1 {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::Gray)
    };

    let history_block = Block::default()
        .title(" Execution History ")
        .borders(Borders::ALL)
        .border_style(history_border_style);
    
    let inner_area = history_block.inner(bottom_chunks[1]);
    frame.render_widget(history_block, bottom_chunks[1]);

    if app.db.history.is_empty() {
        let empty_msg = Paragraph::new("\n  No history logs available yet. Run a command or group sequence to populate.")
            .wrap(Wrap { trim: true });
        frame.render_widget(empty_msg, inner_area);
    } else {
        let mut items = vec![];
        for h in app.db.history.iter().rev() {
            let status_style = if h.status == "OK" {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            };
            
            let title_width = (inner_area.width as usize).saturating_sub(31).min(35).max(8);
            let duration_str = format!(" ({} ms)", h.duration_ms);
            let used_width = h.command_title.chars().count() + duration_str.chars().count();
            let pad_len = title_width.saturating_sub(used_width);
            let pad_spaces = " ".repeat(pad_len);
            let line = Line::from(vec![
                Span::styled(format!("[{}]  ", h.timestamp), Style::default().fg(Color::DarkGray)),
                Span::styled(h.command_title.clone(), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(duration_str, Style::default().fg(Color::Magenta)),
                Span::styled(pad_spaces, Style::default()),
                Span::styled("  ", Style::default()),
                Span::styled(format!("{}", h.status), status_style),
            ]);
            items.push(ListItem::new(line));
        }
        
        let history_widget = List::new(items)
            .block(Block::default().borders(Borders::NONE))
            .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");
            
        let mut history_state = ratatui::widgets::ListState::default();
        if app.dashboard_focused_panel == 1 {
            history_state.select(Some(app.dashboard_history_selected));
        }
        frame.render_stateful_widget(history_widget, inner_area, &mut history_state);
    }
}
