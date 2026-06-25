pub mod forms;
pub mod group_forms;
pub mod navigation;
pub mod lists;

use crate::app::{App, AppScreen};
use ratatui::DefaultTerminal;
use crossterm::event::KeyEvent;

pub fn handle_key_event(app: &mut App, key: KeyEvent, terminal: &mut DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(_) = &app.delete_confirm_title {
        use crossterm::event::KeyCode;
        match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                let title = app.delete_confirm_title.take().unwrap();
                if app.delete_confirm_group {
                    if let Some(pos) = app.db.groups.iter().position(|g| g.name == title) {
                        app.db.groups.remove(pos);
                        let _ = app.db.save();
                    }
                } else {
                    if let Some(pos) = app.db.commands.iter().position(|c| c.title == title) {
                        app.db.commands.remove(pos);
                        let _ = app.db.save();
                    }
                }
                app.list_selected = 0;
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                app.delete_confirm_title = None;
            }
            _ => {}
        }
        return Ok(());
    }

    match app.screen {
        AppScreen::Dashboard => navigation::handle_dashboard_key(app, key)?,
        AppScreen::ListCommands => lists::handle_list_key(app, key, terminal)?,
        AppScreen::UpdateCommandList => lists::handle_update_list_key(app, key)?,
        AppScreen::DeleteCommandList => lists::handle_delete_list_key(app, key)?,
        AppScreen::ExportMenu => navigation::handle_export_key(app, key)?,
        AppScreen::ImportForm => navigation::handle_import_key(app, key)?,
        AppScreen::AddCommand | AppScreen::UpdateCommandForm => {
            forms::handle_form_key_event(app, key)?;
        }
        AppScreen::AddGroup | AppScreen::UpdateGroupForm => {
            group_forms::handle_group_form_key_event(app, key)?;
        }
    }
    Ok(())
}
