use crate::db::{Database, CommandModel, GroupModel};
use crate::cli::get_flag_value;

pub fn handle_cli_command(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.is_empty() {
        eprintln!("Error: Command subcommand required (add, update, delete, list).");
        std::process::exit(1);
    }
    
    let sub = args[0].as_str();
    match sub {
        "add" => {
            let title = get_flag_value(args, "--title").ok_or("Missing --title flag")?;
            let script = get_flag_value(args, "--script").ok_or("Missing --script flag")?;
            let desc = get_flag_value(args, "--desc").unwrap_or_default();
            let group = get_flag_value(args, "--group");
            
            let mut db = Database::load();
            if db.commands.iter().any(|c| c.title == title) {
                return Err(format!("Error: Command with title '{}' already exists.", title).into());
            }
            
            db.commands.push(CommandModel {
                title,
                description: desc,
                script,
                group,
                use_count: 0,
            });
            db.save()?;
            println!("Command added successfully.");
        }
        "update" => {
            let title = get_flag_value(args, "--title").ok_or("Missing --title flag")?;
            let mut db = Database::load();
            let pos = db.commands.iter().position(|c| c.title == title)
                .ok_or_else(|| format!("Error: Command '{}' not found.", title))?;
                
            if let Some(script) = get_flag_value(args, "--script") {
                db.commands[pos].script = script;
            }
            if let Some(desc) = get_flag_value(args, "--desc") {
                db.commands[pos].description = desc;
            }
            if let Some(group) = get_flag_value(args, "--group") {
                db.commands[pos].group = if group.is_empty() { None } else { Some(group) };
            }
            
            db.save()?;
            println!("Command updated successfully.");
        }
        "delete" => {
            let title = get_flag_value(args, "--title").ok_or("Missing --title flag")?;
            let mut db = Database::load();
            let pos = db.commands.iter().position(|c| c.title == title)
                .ok_or_else(|| format!("Error: Command '{}' not found.", title))?;
                
            db.commands.remove(pos);
            db.save()?;
            println!("Command '{}' deleted successfully.", title);
        }
        "list" => {
            let db = Database::load();
            if db.commands.is_empty() {
                println!("No commands stored.");
            } else {
                println!("{:<20} {:<30} {}", "TITLE", "DESCRIPTION", "SCRIPT");
                println!("{}", "-".repeat(70));
                for cmd in &db.commands {
                    let group_str = match &cmd.group {
                        Some(g) => format!(" [{}]", g),
                        None => String::new(),
                    };
                    println!("{:<20} {:<30} {}", format!("{}{}", cmd.title, group_str), cmd.description, cmd.script);
                }
            }
        }
        _ => {
            eprintln!("Unknown command subcommand: {}", sub);
            std::process::exit(1);
        }
    }
    Ok(())
}

pub fn handle_cli_group(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.is_empty() {
        eprintln!("Error: Group subcommand required (add, update, delete, list).");
        std::process::exit(1);
    }
    
    let sub = args[0].as_str();
    match sub {
        "add" => {
            let name = get_flag_value(args, "--name").ok_or("Missing --name flag")?;
            let desc = get_flag_value(args, "--desc").unwrap_or_default();
            let commands_str = get_flag_value(args, "--commands").unwrap_or_default();
            let commands = if commands_str.is_empty() {
                vec![]
            } else {
                commands_str.split(',').map(|s| s.trim().to_string()).collect()
            };
            
            let mut db = Database::load();
            if db.groups.iter().any(|g| g.name == name) {
                return Err(format!("Error: Group '{}' already exists.", name).into());
            }
            
            db.groups.push(GroupModel {
                name,
                description: desc,
                commands,
                use_count: 0,
            });
            db.save()?;
            println!("Group added successfully.");
        }
        "update" => {
            let name = get_flag_value(args, "--name").ok_or("Missing --name flag")?;
            let mut db = Database::load();
            let pos = db.groups.iter().position(|g| g.name == name)
                .ok_or_else(|| format!("Error: Group '{}' not found.", name))?;
                
            if let Some(desc) = get_flag_value(args, "--desc") {
                db.groups[pos].description = desc;
            }
            if let Some(commands_str) = get_flag_value(args, "--commands") {
                db.groups[pos].commands = if commands_str.is_empty() {
                    vec![]
                } else {
                    commands_str.split(',').map(|s| s.trim().to_string()).collect()
                };
            }
            
            db.save()?;
            println!("Group updated successfully.");
        }
        "delete" => {
            let name = get_flag_value(args, "--name").ok_or("Missing --name flag")?;
            let mut db = Database::load();
            let pos = db.groups.iter().position(|g| g.name == name)
                .ok_or_else(|| format!("Error: Group '{}' not found.", name))?;
                
            db.groups.remove(pos);
            db.save()?;
            println!("Group '{}' deleted successfully.", name);
        }
        "list" => {
            let db = Database::load();
            if db.groups.is_empty() {
                println!("No groups stored.");
            } else {
                println!("{:<20} {:<30} {}", "GROUP NAME", "DESCRIPTION", "COMMANDS SEQUENCE");
                println!("{}", "-".repeat(80));
                for g in &db.groups {
                    println!("{:<20} {:<30} {}", g.name, g.description, g.commands.join(", "));
                }
            }
        }
        _ => {
            eprintln!("Unknown group subcommand: {}", sub);
            std::process::exit(1);
        }
    }
    Ok(())
}
