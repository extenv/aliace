pub mod db;
pub mod app;
pub mod cli;
pub mod tui;

use crate::db::Database;
use crate::tui::{
    run_tui_main_dashboard, run_tui_list, run_tui_add, run_tui_update_list,
    run_tui_update_command, run_tui_update_group, run_tui_delete_list,
    run_tui_export, run_tui_import,
};
use crate::cli::{
    handle_cli_command, handle_cli_group, cli_run_command_or_group,
    cli_delete_command, cli_import_command,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        run_tui_main_dashboard()?;
        return Ok(());
    }

    let command = args[1].as_str();
    match command {
        "-v" | "--version" | "version" => {
            println!("v0.1.0");
        }
        "list" => {
            run_tui_list()?;
        }
        "add" => {
            run_tui_add()?;
        }
        "update" => {
            if args.len() >= 3 {
                let target = &args[2];
                let db = Database::load();
                let is_group = db.groups.iter().any(|g| &g.name == target);
                if is_group {
                    run_tui_update_group(target)?;
                } else {
                    run_tui_update_command(target)?;
                }
            } else {
                run_tui_update_list()?;
            }
        }
        "delete" => {
            if args.len() >= 3 {
                cli_delete_command(&args[2])?;
            } else {
                run_tui_delete_list()?;
            }
        }
        "run" => {
            if args.len() >= 3 {
                cli_run_command_or_group(&args[2])?;
            } else {
                eprintln!("Error: Please specify the command title to run. Example: aliace run build");
                std::process::exit(1);
            }
        }
        "export" => {
            run_tui_export()?;
        }
        "import" => {
            if args.len() >= 3 {
                cli_import_command(&args[2])?;
            } else {
                run_tui_import()?;
            }
        }
        "command" => {
            handle_cli_command(&args[2..])?;
        }
        "group" => {
            handle_cli_group(&args[2..])?;
        }
        "help" | "--help" | "-h" => {
            print_help();
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            print_help();
            std::process::exit(1);
        }
    }
    Ok(())
}

fn print_help() {
    println!(
        "Aliace - Command Line Command Manager\n\n\
         Usage:\n\
         Interactive Modes:\n\
           aliace                    Start interactive dashboard TUI\n\
           aliace list               Interactive list & manage screen\n\
           aliace add                Interactive add command screen\n\
           aliace update             Interactive select and edit screen\n\
           aliace update <title>     Interactive edit screen for command or group\n\
           aliace delete             Interactive select and delete screen\n\
           aliace export             Interactive export screen\n\
           aliace import             Interactive import screen\n\n\
         CLI & Scripting Modes:\n\
           aliace run <title>        Execute command or group sequence directly\n\
           aliace delete <title>     Delete command or group with confirmation prompt\n\
           aliace import <path>      Import database backup from JSON file\n\n\
         CLI Commands:\n\
           aliace command add --title <title> --script <script> --desc <desc> [--group <group>]\n\
           aliace command update --title <title> [--script <script>] [--desc <desc>] [--group <group>]\n\
           aliace command delete --title <title>\n\
           aliace command list\n\n\
         CLI Groups:\n\
           aliace group add --name <name> --desc <desc> [--commands <c1,c2,...>]\n\
           aliace group update --name <name> [--desc <desc>] [--commands <c1,c2,...>]\n\
           aliace group delete --name <name>\n\
           aliace group list\n"
    );
}
