use crate::app::{App, AppScreen};
use crate::tui::screens;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let header_title = match &app.screen {
        AppScreen::Dashboard => "🌌 ALIACE - COMMAND CONTROL CENTRE",
        AppScreen::ListCommands => "📋 ALIACE - REGISTRY (COMMANDS & GROUPS)",
        AppScreen::AddCommand => "➕ REGISTER NEW COMMAND",
        AppScreen::AddGroup => "➕ REGISTER NEW GROUP SEQUENCE",
        AppScreen::UpdateCommandList => "✏️ CHOOSE ITEM TO EDIT",
        AppScreen::UpdateCommandForm => "✏️ EDIT COMMAND",
        AppScreen::UpdateGroupForm => "✏️ EDIT GROUP SEQUENCE",
        AppScreen::DeleteCommandList => "❌ CHOOSE ITEM TO DELETE",
        AppScreen::ExportMenu => "📤 EXPORT DATA BACKUP",
        AppScreen::ImportForm => "📥 IMPORT DATA BACKUP",
    };
    
    let cwd = std::env::current_dir()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "Unknown".to_string());

    let header = Paragraph::new(Line::from(vec![
        Span::styled(header_title, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled(" | ", Style::default().fg(Color::DarkGray)),
        Span::styled("CWD: ", Style::default().fg(Color::Gray)),
        Span::styled(cwd, Style::default().fg(Color::Green)),
    ]))
    .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(Style::default().fg(Color::DarkGray)));
    frame.render_widget(header, chunks[0]);

    match &app.screen {
        AppScreen::Dashboard => screens::dashboard::draw_dashboard(frame, chunks[1], app),
        AppScreen::ListCommands | AppScreen::UpdateCommandList | AppScreen::DeleteCommandList => {
            screens::command_list::draw_command_list(frame, chunks[1], app);
        }
        AppScreen::AddCommand | AppScreen::UpdateCommandForm => {
            screens::forms::draw_form(frame, chunks[1], app);
        }
        AppScreen::AddGroup | AppScreen::UpdateGroupForm => {
            screens::forms::draw_group_form(frame, chunks[1], app);
        }
        AppScreen::ExportMenu => {
            screens::backup::draw_export(frame, chunks[1], app);
        }
        AppScreen::ImportForm => {
            screens::backup::draw_import(frame, chunks[1], app);
        }
    }

    let footer_text = match &app.screen {
        AppScreen::Dashboard => {
            Line::from(vec![
                Span::styled(" [L] ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled("List ", Style::default().fg(Color::Gray)),
                Span::styled(" [A] ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled("Add Cmd ", Style::default().fg(Color::Gray)),
                Span::styled(" [G] ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled("Add Group ", Style::default().fg(Color::Gray)),
                Span::styled(" [U] ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled("Update ", Style::default().fg(Color::Gray)),
                Span::styled(" [D] ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled("Delete ", Style::default().fg(Color::Gray)),
                Span::styled(" [E] ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled("Export ", Style::default().fg(Color::Gray)),
                Span::styled(" [I] ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled("Import ", Style::default().fg(Color::Gray)),
                Span::styled(" | ", Style::default().fg(Color::DarkGray)),
                Span::styled(" [Q/Esc] ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::styled("Quit", Style::default().fg(Color::Gray)),
            ])
        }
        AppScreen::ListCommands => {
            Line::from(vec![
                Span::styled(" [←/→] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled("Switch Tab ", Style::default().fg(Color::Gray)),
                Span::styled(" [↑/↓] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled("Select ", Style::default().fg(Color::Gray)),
                Span::styled(" [Enter/R] ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::styled("Run Selected ", Style::default().fg(Color::Gray)),
                Span::styled(" [E] ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled("Edit ", Style::default().fg(Color::Gray)),
                Span::styled(" [D] ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::styled("Delete ", Style::default().fg(Color::Gray)),
                Span::styled(" [A] ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled("Add Cmd ", Style::default().fg(Color::Gray)),
                Span::styled(" [G] ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled("Add Group ", Style::default().fg(Color::Gray)),
                Span::styled(" | ", Style::default().fg(Color::DarkGray)),
                Span::styled(" [Esc] ", Style::default().fg(Color::Gray)),
                Span::styled("Back to Menu", Style::default().fg(Color::Gray)),
            ])
        }
        AppScreen::UpdateCommandList => {
            Line::from(vec![
                Span::styled(" [←/→] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled("Switch Tab ", Style::default().fg(Color::Gray)),
                Span::styled(" [↑/↓] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled("Select ", Style::default().fg(Color::Gray)),
                Span::styled(" [Enter] ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled("Edit Selected ", Style::default().fg(Color::Gray)),
                Span::styled(" | ", Style::default().fg(Color::DarkGray)),
                Span::styled(" [Esc] ", Style::default().fg(Color::Gray)),
                Span::styled("Back to Menu", Style::default().fg(Color::Gray)),
            ])
        }
        AppScreen::DeleteCommandList => {
            Line::from(vec![
                Span::styled(" [←/→] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled("Switch Tab ", Style::default().fg(Color::Gray)),
                Span::styled(" [↑/↓] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled("Select ", Style::default().fg(Color::Gray)),
                Span::styled(" [Enter/D] ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::styled("Delete Selected ", Style::default().fg(Color::Gray)),
                Span::styled(" | ", Style::default().fg(Color::DarkGray)),
                Span::styled(" [Esc] ", Style::default().fg(Color::Gray)),
                Span::styled("Back to Menu", Style::default().fg(Color::Gray)),
            ])
        }
        AppScreen::AddCommand | AppScreen::UpdateCommandForm => {
            Line::from(vec![
                Span::styled(" [Tab/↑/↓] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled("Switch Fields ", Style::default().fg(Color::Gray)),
                Span::styled(" [Char keys] ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled("Type ", Style::default().fg(Color::Gray)),
                Span::styled(" [Enter] ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::styled("Action ", Style::default().fg(Color::Gray)),
                Span::styled(" | ", Style::default().fg(Color::DarkGray)),
                Span::styled(" [Esc] ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::styled("Cancel ", Style::default().fg(Color::Gray)),
            ])
        }
        AppScreen::AddGroup | AppScreen::UpdateGroupForm => {
            Line::from(vec![
                Span::styled(" [Tab/Shift+Tab] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled("Switch Fields ", Style::default().fg(Color::Gray)),
                Span::styled(" [CommandsList]: ", Style::default().fg(Color::DarkGray)),
                Span::styled("↑/↓ Select | Shift+↑/↓ Move position | Backspace Remove", Style::default().fg(Color::Green)),
                Span::styled(" | ", Style::default().fg(Color::DarkGray)),
                Span::styled(" [Avail]: ", Style::default().fg(Color::DarkGray)),
                Span::styled("Enter/A Add to sequence", Style::default().fg(Color::Green)),
            ])
        }
        AppScreen::ExportMenu => {
            Line::from(vec![
                Span::styled(" [↑/↓] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled("Select Type ", Style::default().fg(Color::Gray)),
                Span::styled(" [Enter] ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::styled("Export ", Style::default().fg(Color::Gray)),
                Span::styled(" | ", Style::default().fg(Color::DarkGray)),
                Span::styled(" [Esc] ", Style::default().fg(Color::Gray)),
                Span::styled("Back to Menu", Style::default().fg(Color::Gray)),
            ])
        }
        AppScreen::ImportForm => {
            Line::from(vec![
                Span::styled(" [Char keys] ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled("Type JSON File Path ", Style::default().fg(Color::Gray)),
                Span::styled(" [Enter] ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::styled("Import File ", Style::default().fg(Color::Gray)),
                Span::styled(" | ", Style::default().fg(Color::DarkGray)),
                Span::styled(" [Esc] ", Style::default().fg(Color::Gray)),
                Span::styled("Back to Menu", Style::default().fg(Color::Gray)),
            ])
        }
    };

    let footer = Paragraph::new(footer_text)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(Style::default().fg(Color::DarkGray)));
    frame.render_widget(footer, chunks[2]);

    if let Some(title) = &app.delete_confirm_title {
        render_delete_popup(frame, title, app.delete_confirm_group);
    }
}

fn render_delete_popup(frame: &mut Frame, title: &str, is_group: bool) {
    let area = frame.area();
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(35),
            Constraint::Length(8),
            Constraint::Percentage(35),
        ])
        .split(area);

    let popup_horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(popup_layout[1]);

    let block_area = popup_horizontal[1];
    let type_str = if is_group { "group" } else { "command" };

    let popup_block = Block::default()
        .title(" Confirm Delete ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red))
        .border_type(BorderType::Double);

    let text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(format!("Are you sure you want to delete {} :", type_str), Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(format!("  {}  ", title), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [Y] Yes, Delete  ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::styled("    ", Style::default()),
            Span::styled("  [N/Esc] Cancel  ", Style::default().fg(Color::Gray)),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .block(popup_block)
        .wrap(Wrap { trim: true });

    frame.render_widget(ratatui::widgets::Clear, block_area);
    frame.render_widget(paragraph, block_area);
}
