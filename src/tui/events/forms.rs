use crate::app::{App, AppScreen, FormField};
use crate::db::CommandModel;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_form_key_event(app: &mut App, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Esc => {
            app.screen = AppScreen::Dashboard;
        }
        KeyCode::Tab | KeyCode::Down => {
            app.form_focus = match app.form_focus {
                FormField::Title => FormField::Description,
                FormField::Description => FormField::Script,
                FormField::Script => FormField::Group,
                FormField::Group => FormField::Save,
                FormField::Save => FormField::Cancel,
                FormField::Cancel => FormField::Title,
            };
        }
        KeyCode::Up => {
            app.form_focus = match app.form_focus {
                FormField::Title => FormField::Cancel,
                FormField::Description => FormField::Title,
                FormField::Script => FormField::Description,
                FormField::Group => FormField::Script,
                FormField::Save => FormField::Group,
                FormField::Cancel => FormField::Save,
            };
        }
        KeyCode::Enter => {
            match app.form_focus {
                FormField::Save => {
                    save_form(app);
                }
                FormField::Cancel => {
                    app.screen = AppScreen::Dashboard;
                }
                _ => {
                    app.form_focus = match app.form_focus {
                        FormField::Title => FormField::Description,
                        FormField::Description => FormField::Script,
                        FormField::Script => FormField::Group,
                        FormField::Group => FormField::Save,
                        _ => app.form_focus,
                    };
                }
            }
        }
        KeyCode::Char(c) => {
            let active_str = match app.form_focus {
                FormField::Title => &mut app.form_title,
                FormField::Description => &mut app.form_desc,
                FormField::Script => &mut app.form_script,
                FormField::Group => &mut app.form_group,
                _ => return Ok(()),
            };
            active_str.push(c);
        }
        KeyCode::Backspace => {
            let active_str = match app.form_focus {
                FormField::Title => &mut app.form_title,
                FormField::Description => &mut app.form_desc,
                FormField::Script => &mut app.form_script,
                FormField::Group => &mut app.form_group,
                _ => return Ok(()),
            };
            active_str.pop();
        }
        _ => {}
    }
    Ok(())
}

fn save_form(app: &mut App) {
    let title = app.form_title.trim().to_string();
    let script = app.form_script.trim().to_string();
    let desc = app.form_desc.trim().to_string();
    let group = if app.form_group.trim().is_empty() {
        None
    } else {
        Some(app.form_group.trim().to_string())
    };
    
    if title.is_empty() {
        app.form_error = Some("Title cannot be empty".to_string());
        return;
    }
    if script.is_empty() {
        app.form_error = Some("Command script cannot be empty".to_string());
        return;
    }
    
    match &app.screen {
        AppScreen::AddCommand => {
            if app.db.commands.iter().any(|c| c.title == title) {
                app.form_error = Some(format!("Command with title '{}' already exists", title));
                return;
            }
            app.db.commands.push(CommandModel {
                title,
                description: desc,
                script,
                group,
                use_count: 0,
                favorite: false,
            });
        }
        AppScreen::UpdateCommandForm => {
            let target_title = app.update_target_title.clone();
            if title != target_title && app.db.commands.iter().any(|c| c.title == title) {
                app.form_error = Some(format!("Command with title '{}' already exists", title));
                return;
            }
            if let Some(pos) = app.db.commands.iter().position(|c| c.title == target_title) {
                app.db.commands[pos].title = title;
                app.db.commands[pos].description = desc;
                app.db.commands[pos].script = script;
                app.db.commands[pos].group = group;
            }
        }
        _ => {}
    }
    
    let _ = app.db.save();
    app.screen = AppScreen::ListCommands;
}
