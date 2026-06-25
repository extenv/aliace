use crate::app::{App, AppScreen};
use crate::tui::{run_tui_command_execution, run_tui_group_execution};
use ratatui::DefaultTerminal;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_list_key(app: &mut App, key: KeyEvent, terminal: &mut DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Esc => {
            app.screen = AppScreen::Dashboard;
        }
        KeyCode::Left => {
            if app.list_tab > 0 {
                app.list_tab = 0;
                app.list_selected = 0;
            }
        }
        KeyCode::Right => {
            if app.list_tab < 1 {
                app.list_tab = 1;
                app.list_selected = 0;
            }
        }
        KeyCode::Up => {
            if app.list_selected > 0 {
                app.list_selected -= 1;
            }
        }
        KeyCode::Down => {
            let max_len = if app.list_tab == 0 {
                app.db.commands.len()
            } else {
                app.db.groups.len()
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
            if app.list_tab == 0 {
                if !app.db.commands.is_empty() {
                    let cmd = app.db.commands[app.list_selected].clone();
                    app.init_form_edit(&cmd);
                    app.screen = AppScreen::UpdateCommandForm;
                }
            } else {
                if !app.db.groups.is_empty() {
                    let grp = app.db.groups[app.list_selected].clone();
                    app.init_group_form_edit(&grp);
                    app.screen = AppScreen::UpdateGroupForm;
                }
            }
        }
        KeyCode::Char('d') | KeyCode::Char('D') => {
            if app.list_tab == 0 {
                if !app.db.commands.is_empty() {
                    let cmd = &app.db.commands[app.list_selected];
                    app.delete_confirm_title = Some(cmd.title.clone());
                    app.delete_confirm_group = false;
                }
            } else {
                if !app.db.groups.is_empty() {
                    let grp = &app.db.groups[app.list_selected];
                    app.delete_confirm_title = Some(grp.name.clone());
                    app.delete_confirm_group = true;
                }
            }
        }
        KeyCode::Char('r') | KeyCode::Char('R') | KeyCode::Enter => {
            if app.list_tab == 0 {
                if !app.db.commands.is_empty() {
                    let title = app.db.commands[app.list_selected].title.clone();
                    run_tui_command_execution(terminal, app, title)?;
                }
            } else {
                if !app.db.groups.is_empty() {
                    let name = app.db.groups[app.list_selected].name.clone();
                    run_tui_group_execution(terminal, app, name)?;
                }
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn handle_update_list_key(app: &mut App, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Esc => {
            app.screen = AppScreen::Dashboard;
        }
        KeyCode::Left => {
            if app.list_tab > 0 {
                app.list_tab = 0;
                app.list_selected = 0;
            }
        }
        KeyCode::Right => {
            if app.list_tab < 1 {
                app.list_tab = 1;
                app.list_selected = 0;
            }
        }
        KeyCode::Up => {
            if app.list_selected > 0 {
                app.list_selected -= 1;
            }
        }
        KeyCode::Down => {
            let max_len = if app.list_tab == 0 {
                app.db.commands.len()
            } else {
                app.db.groups.len()
            };
            if max_len > 0 && app.list_selected < max_len - 1 {
                app.list_selected += 1;
            }
        }
        KeyCode::Enter => {
            if app.list_tab == 0 {
                if !app.db.commands.is_empty() {
                    let cmd = app.db.commands[app.list_selected].clone();
                    app.init_form_edit(&cmd);
                    app.screen = AppScreen::UpdateCommandForm;
                }
            } else {
                if !app.db.groups.is_empty() {
                    let grp = app.db.groups[app.list_selected].clone();
                    app.init_group_form_edit(&grp);
                    app.screen = AppScreen::UpdateGroupForm;
                }
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn handle_delete_list_key(app: &mut App, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Esc => {
            app.screen = AppScreen::Dashboard;
        }
        KeyCode::Left => {
            if app.list_tab > 0 {
                app.list_tab = 0;
                app.list_selected = 0;
            }
        }
        KeyCode::Right => {
            if app.list_tab < 1 {
                app.list_tab = 1;
                app.list_selected = 0;
            }
        }
        KeyCode::Up => {
            if app.list_selected > 0 {
                app.list_selected -= 1;
            }
        }
        KeyCode::Down => {
            let max_len = if app.list_tab == 0 {
                app.db.commands.len()
            } else {
                app.db.groups.len()
            };
            if max_len > 0 && app.list_selected < max_len - 1 {
                app.list_selected += 1;
            }
        }
        KeyCode::Enter | KeyCode::Char('d') | KeyCode::Char('D') => {
            if app.list_tab == 0 {
                if !app.db.commands.is_empty() {
                    let cmd = &app.db.commands[app.list_selected];
                    app.delete_confirm_title = Some(cmd.title.clone());
                    app.delete_confirm_group = false;
                }
            } else {
                if !app.db.groups.is_empty() {
                    let grp = &app.db.groups[app.list_selected];
                    app.delete_confirm_title = Some(grp.name.clone());
                    app.delete_confirm_group = true;
                }
            }
        }
        _ => {}
    }
    Ok(())
}
