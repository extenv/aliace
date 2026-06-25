use crate::app::{App, FormField, GroupFormField};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn draw_form(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" Command Editor Form ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Gray));
    
    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .split(inner_area);

    let get_border_style = |field| {
        if app.form_focus == field {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        }
    };

    let title_block = Block::default()
        .title(" Title (Required, Unique) ")
        .borders(Borders::ALL)
        .border_style(get_border_style(FormField::Title));
    let title_para = Paragraph::new(format!(" {}", app.form_title)).block(title_block);
    frame.render_widget(title_para, chunks[0]);

    let desc_block = Block::default()
        .title(" Description ")
        .borders(Borders::ALL)
        .border_style(get_border_style(FormField::Description));
    let desc_para = Paragraph::new(format!(" {}", app.form_desc)).block(desc_block);
    frame.render_widget(desc_para, chunks[1]);

    let script_block = Block::default()
        .title(" Command Script (Required) ")
        .borders(Borders::ALL)
        .border_style(get_border_style(FormField::Script));
    let script_para = Paragraph::new(format!(" {}", app.form_script)).block(script_block);
    frame.render_widget(script_para, chunks[2]);

    let group_block = Block::default()
        .title(" Group (Optional) ")
        .borders(Borders::ALL)
        .border_style(get_border_style(FormField::Group));
    let group_para = Paragraph::new(format!(" {}", app.form_group)).block(group_block);
    frame.render_widget(group_para, chunks[3]);

    let action_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Length(15),
            Constraint::Length(4),
            Constraint::Length(15),
            Constraint::Min(0),
        ])
        .split(chunks[4]);

    let save_style = if app.form_focus == FormField::Save {
        Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Green)
    };
    let save_button = Paragraph::new("    [ SAVE ]    ")
        .style(save_style)
        .block(Block::default().borders(Borders::ALL).border_style(save_style));
    frame.render_widget(save_button, action_chunks[1]);

    let cancel_style = if app.form_focus == FormField::Cancel {
        Style::default().fg(Color::Black).bg(Color::Red).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Red)
    };
    let cancel_button = Paragraph::new("   [ CANCEL ]   ")
        .style(cancel_style)
        .block(Block::default().borders(Borders::ALL).border_style(cancel_style));
    frame.render_widget(cancel_button, action_chunks[3]);

    if let Some(err) = &app.form_error {
        let err_para = Paragraph::new(format!("  ⚠️ Error: {}", err))
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
        frame.render_widget(err_para, chunks[5]);
    }
}

pub fn draw_group_form(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" Group Command Sequence Editor ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Gray));
    
    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Group Name
            Constraint::Length(3), // Group Description
            Constraint::Min(6),    // Sequence & Available Split
            Constraint::Length(3), // Save/Cancel
            Constraint::Length(2), // Error Msg
        ])
        .split(inner_area);

    let get_border_style = |field| {
        if app.group_focus == field {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        }
    };

    // Group Name
    let name_block = Block::default()
        .title(" Group Name (Required, Unique) ")
        .borders(Borders::ALL)
        .border_style(get_border_style(GroupFormField::Name));
    let name_para = Paragraph::new(format!(" {}", app.group_name)).block(name_block);
    frame.render_widget(name_para, chunks[0]);

    // Group Description
    let desc_block = Block::default()
        .title(" Group Description ")
        .borders(Borders::ALL)
        .border_style(get_border_style(GroupFormField::Description));
    let desc_para = Paragraph::new(format!(" {}", app.group_desc)).block(desc_block);
    frame.render_widget(desc_para, chunks[1]);

    // Split for lists
    let list_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[2]);

    // Sequence List (Left)
    let selected_cmds_items: Vec<ListItem> = app.group_commands.iter().enumerate().map(|(i, title)| {
        let style = if app.group_focus == GroupFormField::CommandsList && i == app.group_commands_selected {
            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        ListItem::new(format!("  {}. {}  ", i + 1, title)).style(style)
    }).collect();

    let selected_list_block = Block::default()
        .title(" Execution Sequence (1st run on top) ")
        .borders(Borders::ALL)
        .border_style(get_border_style(GroupFormField::CommandsList));
    let selected_list = List::new(selected_cmds_items)
        .block(selected_list_block);
    frame.render_widget(selected_list, list_chunks[0]);

    // Available Commands List (Right)
    let avail_items: Vec<ListItem> = app.db.commands.iter().enumerate().map(|(i, cmd)| {
        let style = if app.group_focus == GroupFormField::AvailableCommands && i == app.group_avail_selected {
            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        ListItem::new(format!("  + {}  ", cmd.title)).style(style)
    }).collect();

    let avail_list_block = Block::default()
        .title(" Available Registry Commands ")
        .borders(Borders::ALL)
        .border_style(get_border_style(GroupFormField::AvailableCommands));
    let avail_list = List::new(avail_items)
        .block(avail_list_block);
    frame.render_widget(avail_list, list_chunks[1]);

    // Save and Cancel buttons
    let action_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Length(15),
            Constraint::Length(4),
            Constraint::Length(15),
            Constraint::Min(0),
        ])
        .split(chunks[3]);

    let save_style = if app.group_focus == GroupFormField::Save {
        Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Green)
    };
    let save_button = Paragraph::new("    [ SAVE ]    ")
        .style(save_style)
        .block(Block::default().borders(Borders::ALL).border_style(save_style));
    frame.render_widget(save_button, action_chunks[1]);

    let cancel_style = if app.group_focus == GroupFormField::Cancel {
        Style::default().fg(Color::Black).bg(Color::Red).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Red)
    };
    let cancel_button = Paragraph::new("   [ CANCEL ]   ")
        .style(cancel_style)
        .block(Block::default().borders(Borders::ALL).border_style(cancel_style));
    frame.render_widget(cancel_button, action_chunks[3]);

    if let Some(err) = &app.group_error {
        let err_para = Paragraph::new(format!("  ⚠️ Error: {}", err))
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
        frame.render_widget(err_para, chunks[4]);
    }
}
