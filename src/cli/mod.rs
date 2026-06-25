pub mod admin;
pub mod run;

pub use admin::{handle_cli_command, handle_cli_group};
pub use run::{cli_run_command, cli_run_group, cli_run_command_or_group, cli_delete_command};

use crate::db::{Database, CommandModel, GroupModel};

pub fn cli_import_command(path_str: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new(path_str);
    if !path.exists() {
        eprintln!("Error: File not found: {}", path_str);
        std::process::exit(1);
    }
    
    let content = std::fs::read_to_string(path)?;
    let mut db = Database::load();
    
    if let Ok(imported_db) = serde_json::from_str::<Database>(&content) {
        db.commands = imported_db.commands;
        db.groups = imported_db.groups;
        db.history = imported_db.history;
        db.save()?;
        println!("Successfully imported database backup (Everything).");
    } else if let Ok(cmds) = serde_json::from_str::<Vec<CommandModel>>(&content) {
        db.commands = cmds;
        db.save()?;
        println!("Successfully imported commands list.");
    } else if let Ok(grps) = serde_json::from_str::<Vec<GroupModel>>(&content) {
        db.groups = grps;
        db.save()?;
        println!("Successfully imported groups list.");
    } else {
        eprintln!("Error: File is not a valid aliace JSON structure.");
        std::process::exit(1);
    }
    
    Ok(())
}

pub fn get_flag_value(args: &[String], flag: &str) -> Option<String> {
    if let Some(pos) = args.iter().position(|a| a == flag) {
        if pos + 1 < args.len() {
            return Some(args[pos + 1].clone());
        }
    }
    None
}

pub fn get_search_query_from_args(args: &[String]) -> Option<String> {
    if let Some(val) = get_flag_value(args, "--search") {
        return Some(val);
    }
    args.iter().find(|a| !a.starts_with('-')).cloned()
}
