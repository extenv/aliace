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

    // Render Top Cards (3 columns)
    let cards_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(main_chunks[0]);

    let total_commands = app.db.commands.len();
    let total_groups = app.db.groups.len();
    let total_runs = app.db.history.len();

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

    let mut stats_text = vec![
        Line::from(""),
    ];

    if sorted_items.is_empty() {
        stats_text.push(Line::from("  No runs logged yet."));
    } else {
        for (i, item) in top_items.enumerate() {
            let type_tag = if item.is_group { "[Group]  " } else { "[Single] " };
            let type_color = if item.is_group { Color::Magenta } else { Color::Green };
            
            stats_text.push(Line::from(vec![
                Span::styled(format!("  {}. ", i + 1), Style::default().fg(Color::White)),
                Span::styled(type_tag, Style::default().fg(type_color)),
                Span::styled(format!("{:<15}", item.name), Style::default().fg(Color::White)),
                Span::styled(" Used: ", Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{:<4}", item.use_count), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            ]));
        }
    }

    let stats_widget = Paragraph::new(stats_text)
        .block(Block::default().title(" 🔥 Most Run Items ").borders(Borders::ALL).border_style(Style::default().fg(Color::Gray)));
    frame.render_widget(stats_widget, bottom_chunks[0]);

    let history_block = Block::default()
        .title(" Execution History (Last 10) ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Gray));
    
    let inner_area = history_block.inner(bottom_chunks[1]);
    frame.render_widget(history_block, bottom_chunks[1]);

    if app.db.history.is_empty() {
        let empty_msg = Paragraph::new("\n  No history logs available yet. Run a command or group sequence to populate.")
            .wrap(Wrap { trim: true });
        frame.render_widget(empty_msg, inner_area);
    } else {
        let mut items = vec![];
        for h in app.db.history.iter().rev().take(inner_area.height as usize) {
            let status_style = if h.status == "OK" {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            };
            
            let line = Line::from(vec![
                Span::styled(format!("[{}] ", h.timestamp), Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{:<18}", h.command_title), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled("Status: ", Style::default().fg(Color::Gray)),
                Span::styled(format!("{:<7}", h.status), status_style),
                Span::styled(format!("({}ms) ", h.duration_ms), Style::default().fg(Color::Magenta)),
                Span::styled(format!("- {}", h.script), Style::default().fg(Color::Gray)),
            ]);
            items.push(ListItem::new(line));
        }
        
        let history_list = List::new(items)
            .block(Block::default().borders(Borders::NONE));
        frame.render_widget(history_list, inner_area);
    }
}
