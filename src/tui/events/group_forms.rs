use crate::app::{App, AppScreen, GroupFormField};
use crate::db::GroupModel;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_group_form_key_event(app: &mut App, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Esc => {
            app.screen = AppScreen::Dashboard;
        }
        KeyCode::Tab => {
            app.group_focus = match app.group_focus {
                GroupFormField::Name => GroupFormField::Description,
                GroupFormField::Description => GroupFormField::CommandsList,
                GroupFormField::CommandsList => GroupFormField::AvailableCommands,
                GroupFormField::AvailableCommands => GroupFormField::Save,
                GroupFormField::Save => GroupFormField::Cancel,
                GroupFormField::Cancel => GroupFormField::Name,
            };
        }
        KeyCode::BackTab => {
            app.group_focus = match app.group_focus {
                GroupFormField::Name => GroupFormField::Cancel,
                GroupFormField::Description => GroupFormField::Name,
                GroupFormField::CommandsList => GroupFormField::Description,
                GroupFormField::AvailableCommands => GroupFormField::CommandsList,
                GroupFormField::Save => GroupFormField::AvailableCommands,
                GroupFormField::Cancel => GroupFormField::Save,
            };
        }
        KeyCode::Down => {
            match app.group_focus {
                GroupFormField::CommandsList => {
                    if !app.group_commands.is_empty() && app.group_commands_selected < app.group_commands.len() - 1 {
                        if key.modifiers.contains(KeyModifiers::SHIFT) {
                            app.group_commands.swap(app.group_commands_selected, app.group_commands_selected + 1);
                        }
                        app.group_commands_selected += 1;
                    }
                }
                GroupFormField::AvailableCommands => {
                    if !app.db.commands.is_empty() && app.group_avail_selected < app.db.commands.len() - 1 {
                        app.group_avail_selected += 1;
                    }
                }
                _ => {
                    app.group_focus = match app.group_focus {
                        GroupFormField::Name => GroupFormField::Description,
                        GroupFormField::Description => GroupFormField::CommandsList,
                        GroupFormField::CommandsList => GroupFormField::AvailableCommands,
                        GroupFormField::AvailableCommands => GroupFormField::Save,
                        GroupFormField::Save => GroupFormField::Cancel,
                        GroupFormField::Cancel => GroupFormField::Name,
                    };
                }
            }
        }
        KeyCode::Up => {
            match app.group_focus {
                GroupFormField::CommandsList => {
                    if app.group_commands_selected > 0 {
                        if key.modifiers.contains(KeyModifiers::SHIFT) {
                            app.group_commands.swap(app.group_commands_selected, app.group_commands_selected - 1);
                        }
                        app.group_commands_selected -= 1;
                    }
                }
                GroupFormField::AvailableCommands => {
                    if app.group_avail_selected > 0 {
                        app.group_avail_selected -= 1;
                    }
                }
                _ => {
                    app.group_focus = match app.group_focus {
                        GroupFormField::Name => GroupFormField::Cancel,
                        GroupFormField::Description => GroupFormField::Name,
                        GroupFormField::CommandsList => GroupFormField::Description,
                        GroupFormField::AvailableCommands => GroupFormField::CommandsList,
                        GroupFormField::Save => GroupFormField::AvailableCommands,
                        GroupFormField::Cancel => GroupFormField::Save,
                    };
                }
            }
        }
        KeyCode::Enter => {
            match app.group_focus {
                GroupFormField::AvailableCommands => {
                    if !app.db.commands.is_empty() && app.group_avail_selected < app.db.commands.len() {
                        let cmd_title = app.db.commands[app.group_avail_selected].title.clone();
                        app.group_commands.push(cmd_title);
                        app.group_commands_selected = app.group_commands.len() - 1;
                    }
                }
                GroupFormField::Save => {
                    save_group_form(app);
                }
                GroupFormField::Cancel => {
                    app.screen = AppScreen::Dashboard;
                }
                _ => {}
            }
        }
        KeyCode::Backspace | KeyCode::Delete => {
            match app.group_focus {
                GroupFormField::Name => {
                    app.group_name.pop();
                }
                GroupFormField::Description => {
                    app.group_desc.pop();
                }
                GroupFormField::CommandsList => {
                    if !app.group_commands.is_empty() && app.group_commands_selected < app.group_commands.len() {
                        app.group_commands.remove(app.group_commands_selected);
                        if app.group_commands_selected >= app.group_commands.len() && !app.group_commands.is_empty() {
                            app.group_commands_selected = app.group_commands.len() - 1;
                        }
                    }
                }
                _ => {}
            }
        }
        KeyCode::Char(c) => {
            match app.group_focus {
                GroupFormField::Name => {
                    app.group_name.push(c);
                }
                GroupFormField::Description => {
                    app.group_desc.push(c);
                }
                GroupFormField::CommandsList => {
                    if c == 'd' || c == 'D' {
                        if !app.group_commands.is_empty() && app.group_commands_selected < app.group_commands.len() {
                            app.group_commands.remove(app.group_commands_selected);
                            if app.group_commands_selected >= app.group_commands.len() && !app.group_commands.is_empty() {
                                app.group_commands_selected = app.group_commands.len() - 1;
                            }
                        }
                    } else if c == 'k' || c == 'K' {
                        if app.group_commands_selected > 0 && app.group_commands_selected < app.group_commands.len() {
                            app.group_commands.swap(app.group_commands_selected, app.group_commands_selected - 1);
                            app.group_commands_selected -= 1;
                        }
                    } else if c == 'j' || c == 'J' {
                        if app.group_commands_selected + 1 < app.group_commands.len() {
                            app.group_commands.swap(app.group_commands_selected, app.group_commands_selected + 1);
                            app.group_commands_selected += 1;
                        }
                    }
                }
                GroupFormField::AvailableCommands => {
                    if c == 'a' || c == 'A' {
                        if !app.db.commands.is_empty() && app.group_avail_selected < app.db.commands.len() {
                            let cmd_title = app.db.commands[app.group_avail_selected].title.clone();
                            app.group_commands.push(cmd_title);
                            app.group_commands_selected = app.group_commands.len() - 1;
                        }
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}

fn save_group_form(app: &mut App) {
    let name = app.group_name.trim().to_string();
    let desc = app.group_desc.trim().to_string();
    let commands = app.group_commands.clone();
    
    if name.is_empty() {
        app.group_error = Some("Group name cannot be empty".to_string());
        return;
    }
    
    match &app.screen {
        AppScreen::AddGroup => {
            if app.db.groups.iter().any(|g| g.name == name) {
                app.group_error = Some(format!("Group with name '{}' already exists", name));
                return;
            }
            app.db.groups.push(GroupModel {
                name,
                description: desc,
                commands,
                use_count: 0,
                favorite: false,
            });
        }
        AppScreen::UpdateGroupForm => {
            let target_name = app.update_target_group_name.clone();
            if name != target_name && app.db.groups.iter().any(|g| g.name == name) {
                app.group_error = Some(format!("Group with name '{}' already exists", name));
                return;
            }
            if let Some(pos) = app.db.groups.iter().position(|g| g.name == target_name) {
                app.db.groups[pos].name = name;
                app.db.groups[pos].description = desc;
                app.db.groups[pos].commands = commands;
            }
        }
        _ => {}
    }
    
    let _ = app.db.save();
    app.screen = AppScreen::ListCommands;
}
