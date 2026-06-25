pub mod render;
pub mod events;
pub mod screens;

use crate::app::{App, AppScreen};
use crate::db::Database;
use crate::cli::{cli_run_command, cli_run_group};
use ratatui::DefaultTerminal;
use std::time::Duration;
use crossterm::event::{self, Event, KeyEventKind};

pub fn run_tui_app(initial_screen: AppScreen, query: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    let app_result = run_app_loop(&mut terminal, initial_screen, query);
    ratatui::restore();
    app_result
}

pub fn run_tui_main_dashboard() -> Result<(), Box<dyn std::error::Error>> {
    run_tui_app(AppScreen::Dashboard, None)
}

pub fn run_tui_list(query: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    run_tui_app(AppScreen::ListCommands, query)
}

pub fn run_tui_add() -> Result<(), Box<dyn std::error::Error>> {
    run_tui_app(AppScreen::AddCommand, None)
}

pub fn run_tui_update_list(query: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    run_tui_app(AppScreen::UpdateCommandList, query)
}

pub fn run_tui_update_command(_title: &str) -> Result<(), Box<dyn std::error::Error>> {
    run_tui_app(AppScreen::UpdateCommandForm, None)
}

pub fn run_tui_update_group(_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    run_tui_app(AppScreen::UpdateGroupForm, None)
}

pub fn run_tui_delete_list(query: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    run_tui_app(AppScreen::DeleteCommandList, query)
}

pub fn run_tui_export() -> Result<(), Box<dyn std::error::Error>> {
    run_tui_app(AppScreen::ExportMenu, None)
}

pub fn run_tui_import() -> Result<(), Box<dyn std::error::Error>> {
    run_tui_app(AppScreen::ImportForm, None)
}

fn run_app_loop(terminal: &mut DefaultTerminal, initial_screen: AppScreen, query: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new(initial_screen);
    if let Some(q) = query {
        app.list_search_query = q;
    }
    
    if let AppScreen::UpdateCommandForm = initial_screen {
        let args: Vec<String> = std::env::args().collect();
        if args.len() >= 3 {
            let title = &args[2];
            if let Some(pos) = app.db.commands.iter().position(|c| &c.title == title) {
                let cmd = app.db.commands[pos].clone();
                app.init_form_edit(&cmd);
            } else {
                app.init_form_empty();
                app.form_title = title.clone();
            }
        }
    } else if let AppScreen::UpdateGroupForm = initial_screen {
        let args: Vec<String> = std::env::args().collect();
        if args.len() >= 3 {
            let name = &args[2];
            if let Some(pos) = app.db.groups.iter().position(|g| &g.name == name) {
                let grp = app.db.groups[pos].clone();
                app.init_group_form_edit(&grp);
            } else {
                app.init_group_form_empty();
                app.group_name = name.clone();
            }
        }
    }

    while !app.should_quit {
        terminal.draw(|frame| render::draw(frame, &mut app))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    events::handle_key_event(&mut app, key, terminal)?;
                }
            }
        } else {
            app.tick_count += 1;
        }
    }
    Ok(())
}

pub fn run_tui_command_execution(terminal: &mut DefaultTerminal, app: &mut App, title: String) -> Result<(), Box<dyn std::error::Error>> {
    ratatui::restore();
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
    
    let _ = cli_run_command(&title);
    
    println!("\nPress Enter to return to Aliace...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    
    *terminal = ratatui::init();
    terminal.clear()?;
    
    app.db = Database::load();
    Ok(())
}

pub fn run_tui_group_execution(terminal: &mut DefaultTerminal, app: &mut App, name: String) -> Result<(), Box<dyn std::error::Error>> {
    ratatui::restore();
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
    
    let _ = cli_run_group(&name);
    
    println!("\nPress Enter to return to Aliace...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    
    *terminal = ratatui::init();
    terminal.clear()?;
    
    app.db = Database::load();
    Ok(())
}
