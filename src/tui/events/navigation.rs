use crate::app::{App, AppScreen};
use crate::db::Database;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_dashboard_key(app: &mut App, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
            app.should_quit = true;
        }
        KeyCode::Char('l') | KeyCode::Char('L') => {
            app.screen = AppScreen::ListCommands;
            app.list_selected = 0;
            app.list_tab = 0;
        }
        KeyCode::Char('a') | KeyCode::Char('A') => {
            app.init_form_empty();
            app.screen = AppScreen::AddCommand;
        }
        KeyCode::Char('g') | KeyCode::Char('G') => {
            app.init_group_form_empty();
            app.screen = AppScreen::AddGroup;
        }
        KeyCode::Char('u') | KeyCode::Char('U') => {
            app.screen = AppScreen::UpdateCommandList;
            app.list_selected = 0;
            app.list_tab = 0;
        }
        KeyCode::Char('d') | KeyCode::Char('D') => {
            app.screen = AppScreen::DeleteCommandList;
            app.list_selected = 0;
            app.list_tab = 0;
        }
        KeyCode::Char('e') | KeyCode::Char('E') => {
            app.screen = AppScreen::ExportMenu;
            app.export_selected = 0;
            app.export_message = None;
        }
        KeyCode::Char('i') | KeyCode::Char('I') => {
            app.screen = AppScreen::ImportForm;
            app.import_path = String::new();
            app.import_message = None;
        }
        _ => {}
    }
    Ok(())
}

pub fn handle_export_key(app: &mut App, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Esc => {
            app.screen = AppScreen::Dashboard;
        }
        KeyCode::Up => {
            if app.export_selected > 0 {
                app.export_selected -= 1;
            }
        }
        KeyCode::Down => {
            if app.export_selected < 2 {
                app.export_selected += 1;
            }
        }
        KeyCode::Enter => {
            let export_type = match app.export_selected {
                0 => "commands",
                1 => "groups",
                _ => "everything",
            };
            let file_name = "aliace_backup.json";
            let res = match export_type {
                "commands" => {
                    serde_json::to_string_pretty(&app.db.commands)
                        .map_err(|e| e.to_string())
                        .and_then(|j| std::fs::write(file_name, j).map_err(|e| e.to_string()))
                }
                "groups" => {
                    serde_json::to_string_pretty(&app.db.groups)
                        .map_err(|e| e.to_string())
                        .and_then(|j| std::fs::write(file_name, j).map_err(|e| e.to_string()))
                }
                _ => {
                    serde_json::to_string_pretty(&app.db)
                        .map_err(|e| e.to_string())
                        .and_then(|j| std::fs::write(file_name, j).map_err(|e| e.to_string()))
                }
            };
            match res {
                Ok(_) => app.export_message = Some(format!("Exported successfully to {}", file_name)),
                Err(e) => app.export_message = Some(format!("Error: {}", e)),
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn handle_import_key(app: &mut App, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Esc => {
            app.screen = AppScreen::Dashboard;
        }
        KeyCode::Char(c) => {
            app.import_path.push(c);
        }
        KeyCode::Backspace => {
            app.import_path.pop();
        }
        KeyCode::Enter => {
            let path_str = app.import_path.trim();
            let path = std::path::Path::new(path_str);
            if !path.exists() {
                app.import_message = Some("Error: File not found".to_string());
            } else {
                match std::fs::read_to_string(path) {
                    Ok(content) => {
                        if let Ok(imported_db) = serde_json::from_str::<Database>(&content) {
                            app.db.commands = imported_db.commands;
                            app.db.groups = imported_db.groups;
                            app.db.history = imported_db.history;
                            let _ = app.db.save();
                            app.import_message = Some("Imported Everything successfully!".to_string());
                        } else if let Ok(cmds) = serde_json::from_str::<Vec<crate::db::CommandModel>>(&content) {
                            app.db.commands = cmds;
                            let _ = app.db.save();
                            app.import_message = Some("Imported Commands list successfully!".to_string());
                        } else if let Ok(grps) = serde_json::from_str::<Vec<crate::db::GroupModel>>(&content) {
                            app.db.groups = grps;
                            let _ = app.db.save();
                            app.import_message = Some("Imported Groups list successfully!".to_string());
                        } else {
                            app.import_message = Some("Error: Invalid JSON format".to_string());
                        }
                    }
                    Err(e) => {
                        app.import_message = Some(format!("Error: {}", e));
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}
