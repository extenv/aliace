use crate::db::{Database, HistoryModel};

pub fn cli_run_command(title: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut db = Database::load();
    let cmd_index = db.commands.iter().position(|c| c.title == title);
    
    let cmd = match cmd_index {
        Some(idx) => &mut db.commands[idx],
        None => {
            eprintln!("Error: Command '{}' not found.", title);
            std::process::exit(1);
        }
    };
    
    println!("Running command '{}': {}", cmd.title, cmd.script);
    cmd.use_count += 1;
    let script = cmd.script.clone();
    db.save()?;
    
    let start_time = std::time::Instant::now();
    
    let status = if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(&["/C", &script])
            .status()
    } else {
        std::process::Command::new("sh")
            .args(&["-c", &script])
            .status()
    };
    
    let duration = start_time.elapsed();
    let status_str = match &status {
        Ok(s) if s.success() => "OK",
        _ => "FAILED",
    };
    
    let mut db = Database::load();
    if let Some(idx) = db.commands.iter_mut().position(|c| c.title == title) {
        db.commands[idx].use_count += 1;
    }
    
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    db.history.push(HistoryModel {
        command_title: title.to_string(),
        script,
        timestamp,
        duration_ms: duration.as_millis() as u64,
        status: status_str.to_string(),
    });
    
    if db.history.len() > 100 {
        db.history.remove(0);
    }
    db.save()?;
    
    println!("\nExecution completed with status: {}", status_str);
    Ok(())
}

pub fn cli_run_group(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut db = Database::load();
    let group_index = db.groups.iter().position(|g| g.name == name);
    
    let group = match group_index {
        Some(idx) => &mut db.groups[idx],
        None => {
            eprintln!("Error: Group '{}' not found.", name);
            std::process::exit(1);
        }
    };
    
    println!("Running group '{}' containing {} commands:", group.name, group.commands.len());
    group.use_count += 1;
    let commands = group.commands.clone();
    db.save()?;
    
    let group_start_time = std::time::Instant::now();
    let mut success_count = 0;
    let mut failed_count = 0;
    
    for (i, cmd_title) in commands.iter().enumerate() {
        println!("\n[{}/{}] Executing '{}'...", i + 1, commands.len(), cmd_title);
        
        let mut db_reload = Database::load();
        let cmd_opt = db_reload.commands.iter_mut().find(|c| &c.title == cmd_title);
        
        if let Some(cmd) = cmd_opt {
            cmd.use_count += 1;
            let script = cmd.script.clone();
            let _ = db_reload.save();
            
            let start_time = std::time::Instant::now();
            let status = if cfg!(target_os = "windows") {
                std::process::Command::new("cmd")
                    .args(&["/C", &script])
                    .status()
            } else {
                std::process::Command::new("sh")
                    .args(&["-c", &script])
                    .status()
            };
            
            let duration = start_time.elapsed();
            let status_str = match &status {
                Ok(s) if s.success() => {
                    success_count += 1;
                    "OK"
                }
                _ => {
                    failed_count += 1;
                    "FAILED"
                }
            };
            
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            db_reload.history.push(HistoryModel {
                command_title: format!("{} -> {}", name, cmd_title),
                script,
                timestamp,
                duration_ms: duration.as_millis() as u64,
                status: status_str.to_string(),
            });
            
            if db_reload.history.len() > 100 {
                db_reload.history.remove(0);
            }
            let _ = db_reload.save();
            
            println!("Command '{}' completed with status: {}", cmd_title, status_str);
        } else {
            failed_count += 1;
            println!("Error: Command '{}' not found in database.", cmd_title);
        }
    }
    
    let group_duration = group_start_time.elapsed();
    println!(
        "\nGroup '{}' execution completed in {}ms. Status: {} OK, {} FAILED.",
        name,
        group_duration.as_millis(),
        success_count,
        failed_count
    );
    
    let mut db_final = Database::load();
    let status_summary = if failed_count == 0 { "OK" } else { "FAILED" };
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    db_final.history.push(HistoryModel {
        command_title: name.to_string(),
        script: format!("Execute group of {} commands", commands.len()),
        timestamp,
        duration_ms: group_duration.as_millis() as u64,
        status: status_summary.to_string(),
    });
    if db_final.history.len() > 100 {
        db_final.history.remove(0);
    }
    let _ = db_final.save();
    
    Ok(())
}

pub fn cli_run_command_or_group(title_or_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::load();
    if db.commands.iter().any(|c| c.title == title_or_name) {
        cli_run_command(title_or_name)?;
    } else if db.groups.iter().any(|g| g.name == title_or_name) {
        cli_run_group(title_or_name)?;
    } else {
        eprintln!("Error: Command or Group '{}' not found.", title_or_name);
        std::process::exit(1);
    }
    Ok(())
}

pub fn cli_delete_command(title_or_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::load();
    let is_command = db.commands.iter().any(|c| c.title == title_or_name);
    let is_group = db.groups.iter().any(|g| g.name == title_or_name);
    
    if !is_command && !is_group {
        eprintln!("Error: Command or Group '{}' not found.", title_or_name);
        std::process::exit(1);
    }
    
    let type_str = if is_command { "command" } else { "group" };
    print!("Are you sure you want to delete {} '{}'? (y/N): ", type_str, title_or_name);
    use std::io::Write;
    std::io::stdout().flush()?;
    let mut response = String::new();
    std::io::stdin().read_line(&mut response)?;
    
    if response.trim().eq_ignore_ascii_case("y") {
        let mut db = Database::load();
        if is_command {
            if let Some(idx) = db.commands.iter().position(|c| c.title == title_or_name) {
                db.commands.remove(idx);
                db.save()?;
                println!("Command deleted successfully.");
            }
        } else {
            if let Some(idx) = db.groups.iter().position(|g| g.name == title_or_name) {
                db.groups.remove(idx);
                db.save()?;
                println!("Group deleted successfully.");
            }
        }
    } else {
        println!("Deletion cancelled.");
    }
    Ok(())
}
