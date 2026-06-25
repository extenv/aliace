use crate::db::{Database, CommandModel, GroupModel};
use crate::cli::{get_flag_value, get_search_query_from_args};

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
                favorite: false,
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
            let search_query = get_search_query_from_args(&args[1..]);
            let most_run = args.iter().any(|a| a == "--most-run");
            
            let mut filtered: Vec<CommandModel> = db.commands.clone();
            
            if let Some(ref q) = search_query {
                let q_lower = q.to_lowercase();
                filtered.retain(|cmd| {
                    cmd.title.to_lowercase().contains(&q_lower)
                        || cmd.description.to_lowercase().contains(&q_lower)
                        || cmd.script.to_lowercase().contains(&q_lower)
                });
            }
            
            if most_run {
                filtered.sort_by(|a, b| b.use_count.cmp(&a.use_count));
            }
            
            if filtered.is_empty() {
                if search_query.is_some() {
                    println!("No matching commands found.");
                } else {
                    println!("No commands stored.");
                }
            } else {
                println!("{:<20} {:<12} {:<30} {}", "TITLE", "RUNS", "DESCRIPTION", "SCRIPT");
                println!("{}", "-".repeat(80));
                for cmd in &filtered {
                    let group_str = match &cmd.group {
                        Some(g) => format!(" [{}]", g),
                        None => String::new(),
                    };
                    let fav_star = if cmd.favorite { " ★" } else { "" };
                    println!("{:<20} {:<12} {:<30} {}", format!("{}{}{}", cmd.title, group_str, fav_star), cmd.use_count, cmd.description, cmd.script);
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
                favorite: false,
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
            let search_query = get_search_query_from_args(&args[1..]);
            let most_run = args.iter().any(|a| a == "--most-run");
            
            let mut filtered: Vec<GroupModel> = db.groups.clone();
            
            if let Some(ref q) = search_query {
                let q_lower = q.to_lowercase();
                filtered.retain(|g| {
                    g.name.to_lowercase().contains(&q_lower)
                        || g.description.to_lowercase().contains(&q_lower)
                        || g.commands.iter().any(|c| c.to_lowercase().contains(&q_lower))
                });
            }
            
            if most_run {
                filtered.sort_by(|a, b| b.use_count.cmp(&a.use_count));
            }
            
            if filtered.is_empty() {
                if search_query.is_some() {
                    println!("No matching groups found.");
                } else {
                    println!("No groups stored.");
                }
            } else {
                println!("{:<20} {:<12} {:<30} {}", "GROUP NAME", "RUNS", "DESCRIPTION", "COMMANDS SEQUENCE");
                println!("{}", "-".repeat(90));
                for g in &filtered {
                    let fav_star = if g.favorite { " ★" } else { "" };
                    println!("{:<20} {:<12} {:<30} {}", format!("{}{}", g.name, fav_star), g.use_count, g.description, g.commands.join(", "));
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
