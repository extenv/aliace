use crate::app::{App, AppScreen};
use crate::tui::{run_tui_command_execution, run_tui_group_execution};
use ratatui::DefaultTerminal;
use crossterm::event::{KeyCode, KeyEvent};

fn handle_search_key(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.list_is_searching = false;
        }
        KeyCode::Char(c) => {
            app.list_search_query.push(c);
            app.list_selected = 0;
        }
        KeyCode::Backspace => {
            app.list_search_query.pop();
            app.list_selected = 0;
        }
        KeyCode::Left => {
            if app.list_tab > 0 {
                app.list_tab -= 1;
                app.list_selected = 0;
            }
        }
        KeyCode::Right => {
            if app.list_tab < 2 {
                app.list_tab += 1;
                app.list_selected = 0;
            }
        }
        KeyCode::Up => {
            if app.list_selected > 0 {
                app.list_selected -= 1;
            }
        }
        KeyCode::Down => {
            let max_len = match app.list_tab {
                0 => app.get_filtered_commands().len(),
                1 => app.get_filtered_groups().len(),
                _ => app.get_filtered_favorites().len(),
            };
            if max_len > 0 && app.list_selected < max_len - 1 {
                app.list_selected += 1;
            }
        }
        _ => {}
    }
}

fn swap_commands(app: &mut App, idx1: usize, idx2: usize) {
    let filtered = app.get_filtered_commands();
    if idx1 < filtered.len() && idx2 < filtered.len() {
        let title1 = &filtered[idx1].title;
        let title2 = &filtered[idx2].title;
        if let (Some(db_idx1), Some(db_idx2)) = (
            app.db.commands.iter().position(|c| &c.title == title1),
            app.db.commands.iter().position(|c| &c.title == title2),
        ) {
            app.db.commands.swap(db_idx1, db_idx2);
            let _ = app.db.save();
        }
    }
}

fn swap_groups(app: &mut App, idx1: usize, idx2: usize) {
    let filtered = app.get_filtered_groups();
    if idx1 < filtered.len() && idx2 < filtered.len() {
        let name1 = &filtered[idx1].name;
        let name2 = &filtered[idx2].name;
        if let (Some(db_idx1), Some(db_idx2)) = (
            app.db.groups.iter().position(|g| &g.name == name1),
            app.db.groups.iter().position(|g| &g.name == name2),
        ) {
            app.db.groups.swap(db_idx1, db_idx2);
            let _ = app.db.save();
        }
    }
}

fn swap_favorites(app: &mut App, idx1: usize, idx2: usize) {
    let filtered = app.get_filtered_favorites();
    if idx1 < filtered.len() && idx2 < filtered.len() {
        let item1 = &filtered[idx1];
        let item2 = &filtered[idx2];
        
        if item1.is_group == item2.is_group {
            if item1.is_group {
                if let (Some(db_idx1), Some(db_idx2)) = (
                    app.db.groups.iter().position(|g| g.name == item1.name),
                    app.db.groups.iter().position(|g| g.name == item2.name),
                ) {
                    app.db.groups.swap(db_idx1, db_idx2);
                    let _ = app.db.save();
                }
            } else {
                if let (Some(db_idx1), Some(db_idx2)) = (
                    app.db.commands.iter().position(|c| c.title == item1.name),
                    app.db.commands.iter().position(|c| c.title == item2.name),
                ) {
                    app.db.commands.swap(db_idx1, db_idx2);
                    let _ = app.db.save();
                }
            }
        }
    }
}

fn handle_list_common_keys(app: &mut App, key: KeyEvent) -> Result<bool, Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Tab => {
            if app.list_grabbed.is_some() {
                app.list_grabbed = None;
            } else {
                let max_len = match app.list_tab {
                    0 => app.get_filtered_commands().len(),
                    1 => app.get_filtered_groups().len(),
                    _ => app.get_filtered_favorites().len(),
                };
                if max_len > 0 {
                    app.list_grabbed = Some(app.list_selected);
                }
            }
            Ok(true)
        }
        KeyCode::Char('f') | KeyCode::Char('F') => {
            let filtered_commands = app.get_filtered_commands();
            let filtered_groups = app.get_filtered_groups();
            let filtered_favorites = app.get_filtered_favorites();
            
            if app.list_tab == 0 {
                if !filtered_commands.is_empty() {
                    let cmd = &filtered_commands[app.list_selected];
                    app.toggle_favorite(&cmd.title, false);
                }
            } else if app.list_tab == 1 {
                if !filtered_groups.is_empty() {
                    let grp = &filtered_groups[app.list_selected];
                    app.toggle_favorite(&grp.name, true);
                }
            } else {
                if !filtered_favorites.is_empty() {
                    let item = &filtered_favorites[app.list_selected];
                    app.toggle_favorite(&item.name, item.is_group);
                }
            }
            Ok(true)
        }
        KeyCode::Up => {
            if let Some(grabbed_idx) = app.list_grabbed {
                if grabbed_idx > 0 {
                    let new_idx = grabbed_idx - 1;
                    if app.list_tab == 0 {
                        swap_commands(app, grabbed_idx, new_idx);
                    } else if app.list_tab == 1 {
                        swap_groups(app, grabbed_idx, new_idx);
                    } else {
                        swap_favorites(app, grabbed_idx, new_idx);
                    }
                    app.list_selected = new_idx;
                    app.list_grabbed = Some(new_idx);
                }
                Ok(true)
            } else {
                Ok(false)
            }
        }
        KeyCode::Down => {
            if let Some(grabbed_idx) = app.list_grabbed {
                let max_len = match app.list_tab {
                    0 => app.get_filtered_commands().len(),
                    1 => app.get_filtered_groups().len(),
                    _ => app.get_filtered_favorites().len(),
                };
                if grabbed_idx < max_len - 1 {
                    let new_idx = grabbed_idx + 1;
                    if app.list_tab == 0 {
                        swap_commands(app, grabbed_idx, new_idx);
                    } else if app.list_tab == 1 {
                        swap_groups(app, grabbed_idx, new_idx);
                    } else {
                        swap_favorites(app, grabbed_idx, new_idx);
                    }
                    app.list_selected = new_idx;
                    app.list_grabbed = Some(new_idx);
                }
                Ok(true)
            } else {
                Ok(false)
            }
        }
        KeyCode::Esc => {
            if app.list_grabbed.is_some() {
                app.list_grabbed = None;
                Ok(true)
            } else {
                Ok(false)
            }
        }
        _ => Ok(false),
    }
}

pub fn handle_list_key(app: &mut App, key: KeyEvent, terminal: &mut DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
    if app.list_is_searching {
        if key.code == KeyCode::Enter {
            app.list_is_searching = false;
        } else {
            handle_search_key(app, key);
            return Ok(());
        }
    }

    if handle_list_common_keys(app, key)? {
        return Ok(());
    }

    match key.code {
        KeyCode::Esc => {
            if !app.list_search_query.is_empty() {
                app.list_search_query.clear();
                app.list_selected = 0;
            } else {
                app.screen = AppScreen::Dashboard;
            }
        }
        KeyCode::Char('/') => {
            app.list_is_searching = true;
        }
        KeyCode::Left => {
            if app.list_tab > 0 {
                app.list_tab -= 1;
                app.list_selected = 0;
                app.list_grabbed = None;
            }
        }
        KeyCode::Right => {
            if app.list_tab < 2 {
                app.list_tab += 1;
                app.list_selected = 0;
                app.list_grabbed = None;
            }
        }
        KeyCode::Up => {
            if app.list_selected > 0 {
                app.list_selected -= 1;
            }
        }
        KeyCode::Down => {
            let max_len = match app.list_tab {
                0 => app.get_filtered_commands().len(),
                1 => app.get_filtered_groups().len(),
                _ => app.get_filtered_favorites().len(),
            };
            if max_len > 0 && app.list_selected < max_len - 1 {
                app.list_selected += 1;
            }
        }
        KeyCode::Char('a') | KeyCode::Char('A') => {
            app.init_form_empty();
            app.screen = AppScreen::AddCommand;
        }
        KeyCode::Char('g') | KeyCode::Char('G') => {
            app.init_group_form_empty();
            app.screen = AppScreen::AddGroup;
        }
        KeyCode::Char('e') | KeyCode::Char('E') => {
            let filtered_commands = app.get_filtered_commands();
            let filtered_groups = app.get_filtered_groups();
            let filtered_favorites = app.get_filtered_favorites();
            if app.list_tab == 0 {
                if !filtered_commands.is_empty() {
                    let cmd = &filtered_commands[app.list_selected];
                    app.init_form_edit(cmd);
                    app.screen = AppScreen::UpdateCommandForm;
                }
            } else if app.list_tab == 1 {
                if !filtered_groups.is_empty() {
                    let grp = &filtered_groups[app.list_selected];
                    app.init_group_form_edit(grp);
                    app.screen = AppScreen::UpdateGroupForm;
                }
            } else {
                if !filtered_favorites.is_empty() {
                    let item = &filtered_favorites[app.list_selected];
                    if item.is_group {
                        if let Some(grp) = app.db.groups.iter().find(|g| g.name == item.name).cloned() {
                            app.init_group_form_edit(&grp);
                            app.screen = AppScreen::UpdateGroupForm;
                        }
                    } else {
                        if let Some(cmd) = app.db.commands.iter().find(|c| c.title == item.name).cloned() {
                            app.init_form_edit(&cmd);
                            app.screen = AppScreen::UpdateCommandForm;
                        }
                    }
                }
            }
        }
        KeyCode::Char('d') | KeyCode::Char('D') => {
            let filtered_commands = app.get_filtered_commands();
            let filtered_groups = app.get_filtered_groups();
            let filtered_favorites = app.get_filtered_favorites();
            if app.list_tab == 0 {
                if !filtered_commands.is_empty() {
                    let cmd = &filtered_commands[app.list_selected];
                    app.delete_confirm_title = Some(cmd.title.clone());
                    app.delete_confirm_group = false;
                }
            } else if app.list_tab == 1 {
                if !filtered_groups.is_empty() {
                    let grp = &filtered_groups[app.list_selected];
                    app.delete_confirm_title = Some(grp.name.clone());
                    app.delete_confirm_group = true;
                }
            } else {
                if !filtered_favorites.is_empty() {
                    let item = &filtered_favorites[app.list_selected];
                    app.delete_confirm_title = Some(item.name.clone());
                    app.delete_confirm_group = item.is_group;
                }
            }
        }
        KeyCode::Char('r') | KeyCode::Char('R') | KeyCode::Enter => {
            let filtered_commands = app.get_filtered_commands();
            let filtered_groups = app.get_filtered_groups();
            let filtered_favorites = app.get_filtered_favorites();
            if app.list_tab == 0 {
                if !filtered_commands.is_empty() {
                    let title = filtered_commands[app.list_selected].title.clone();
                    run_tui_command_execution(terminal, app, title)?;
                }
            } else if app.list_tab == 1 {
                if !filtered_groups.is_empty() {
                    let name = filtered_groups[app.list_selected].name.clone();
                    run_tui_group_execution(terminal, app, name)?;
                }
            } else {
                if !filtered_favorites.is_empty() {
                    let item = &filtered_favorites[app.list_selected];
                    let title_or_name = item.name.clone();
                    if item.is_group {
                        run_tui_group_execution(terminal, app, title_or_name)?;
                    } else {
                        run_tui_command_execution(terminal, app, title_or_name)?;
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn handle_update_list_key(app: &mut App, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    if app.list_is_searching {
        if key.code == KeyCode::Enter {
            app.list_is_searching = false;
        } else {
            handle_search_key(app, key);
            return Ok(());
        }
    }

    if handle_list_common_keys(app, key)? {
        return Ok(());
    }

    match key.code {
        KeyCode::Esc => {
            if !app.list_search_query.is_empty() {
                app.list_search_query.clear();
                app.list_selected = 0;
            } else {
                app.screen = AppScreen::Dashboard;
            }
        }
        KeyCode::Char('/') => {
            app.list_is_searching = true;
        }
        KeyCode::Left => {
            if app.list_tab > 0 {
                app.list_tab -= 1;
                app.list_selected = 0;
                app.list_grabbed = None;
            }
        }
        KeyCode::Right => {
            if app.list_tab < 2 {
                app.list_tab += 1;
                app.list_selected = 0;
                app.list_grabbed = None;
            }
        }
        KeyCode::Up => {
            if app.list_selected > 0 {
                app.list_selected -= 1;
            }
        }
        KeyCode::Down => {
            let max_len = match app.list_tab {
                0 => app.get_filtered_commands().len(),
                1 => app.get_filtered_groups().len(),
                _ => app.get_filtered_favorites().len(),
            };
            if max_len > 0 && app.list_selected < max_len - 1 {
                app.list_selected += 1;
            }
        }
        KeyCode::Enter | KeyCode::Char('e') | KeyCode::Char('E') => {
            let filtered_commands = app.get_filtered_commands();
            let filtered_groups = app.get_filtered_groups();
            let filtered_favorites = app.get_filtered_favorites();
            if app.list_tab == 0 {
                if !filtered_commands.is_empty() {
                    let cmd = &filtered_commands[app.list_selected];
                    app.init_form_edit(cmd);
                    app.screen = AppScreen::UpdateCommandForm;
                }
            } else if app.list_tab == 1 {
                if !filtered_groups.is_empty() {
                    let grp = &filtered_groups[app.list_selected];
                    app.init_group_form_edit(grp);
                    app.screen = AppScreen::UpdateGroupForm;
                }
            } else {
                if !filtered_favorites.is_empty() {
                    let item = &filtered_favorites[app.list_selected];
                    if item.is_group {
                        if let Some(grp) = app.db.groups.iter().find(|g| g.name == item.name).cloned() {
                            app.init_group_form_edit(&grp);
                            app.screen = AppScreen::UpdateGroupForm;
                        }
                    } else {
                        if let Some(cmd) = app.db.commands.iter().find(|c| c.title == item.name).cloned() {
                            app.init_form_edit(&cmd);
                            app.screen = AppScreen::UpdateCommandForm;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn handle_delete_list_key(app: &mut App, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    if app.list_is_searching {
        if key.code == KeyCode::Enter {
            app.list_is_searching = false;
        } else {
            handle_search_key(app, key);
            return Ok(());
        }
    }

    if handle_list_common_keys(app, key)? {
        return Ok(());
    }

    match key.code {
        KeyCode::Esc => {
            if !app.list_search_query.is_empty() {
                app.list_search_query.clear();
                app.list_selected = 0;
            } else {
                app.screen = AppScreen::Dashboard;
            }
        }
        KeyCode::Char('/') => {
            app.list_is_searching = true;
        }
        KeyCode::Left => {
            if app.list_tab > 0 {
                app.list_tab -= 1;
                app.list_selected = 0;
                app.list_grabbed = None;
            }
        }
        KeyCode::Right => {
            if app.list_tab < 2 {
                app.list_tab += 1;
                app.list_selected = 0;
                app.list_grabbed = None;
            }
        }
        KeyCode::Up => {
            if app.list_selected > 0 {
                app.list_selected -= 1;
            }
        }
        KeyCode::Down => {
            let max_len = match app.list_tab {
                0 => app.get_filtered_commands().len(),
                1 => app.get_filtered_groups().len(),
                _ => app.get_filtered_favorites().len(),
            };
            if max_len > 0 && app.list_selected < max_len - 1 {
                app.list_selected += 1;
            }
        }
        KeyCode::Enter | KeyCode::Char('d') | KeyCode::Char('D') => {
            let filtered_commands = app.get_filtered_commands();
            let filtered_groups = app.get_filtered_groups();
            let filtered_favorites = app.get_filtered_favorites();
            if app.list_tab == 0 {
                if !filtered_commands.is_empty() {
                    let cmd = &filtered_commands[app.list_selected];
                    app.delete_confirm_title = Some(cmd.title.clone());
                    app.delete_confirm_group = false;
                }
            } else if app.list_tab == 1 {
                if !filtered_groups.is_empty() {
                    let grp = &filtered_groups[app.list_selected];
                    app.delete_confirm_title = Some(grp.name.clone());
                    app.delete_confirm_group = true;
                }
            } else {
                if !filtered_favorites.is_empty() {
                    let item = &filtered_favorites[app.list_selected];
                    app.delete_confirm_title = Some(item.name.clone());
                    app.delete_confirm_group = item.is_group;
                }
            }
        }
        _ => {}
    }
    Ok(())
}
