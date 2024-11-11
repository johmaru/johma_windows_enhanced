use clap::{Parser, Subcommand};
use std::{env::consts::OS, fs};
use sysinfo::{Components, Disks, Networks, System};
use tabled::{builder::Builder, settings::Style};

#[derive(Parser)]
#[command(version, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand)]
enum Commands {
    #[command(
        about = "Show memory information",
        long_about = "Show memory information"
    )]
    Mem {
        #[command(subcommand)]
        action: Option<MemShowCommands>,
    },
    Ls {
        #[arg(long)]
        action: bool,
    },
}

#[derive(Subcommand)]
enum MemShowCommands {
    #[command(
        about = "Show memory information",
        long_about = "Show memory information"
    )]
    Show {
        #[arg(short, long, help = "Show total memory information")]
        all: bool,
        #[arg(short, long, help = "Show free memory information")]
        free: bool,
        #[arg(short, long, help = "Show used memory information")]
        used: bool,
        #[arg(short, long, help = "Show available memory information")]
        available: bool,
    },
}
fn main() {
    let args = Args::parse();

    match OS {
        "windows" => {
            windows_cmd(args);
        }
        _ => {
            println!("Unsupported OS");
        }
    }
}

fn windows_cmd(args: Args) {
    match &args.command {
        Some(Commands::Mem { action }) => {
            let mut sys = System::new_all();
            sys.refresh_all();
            match action {
                Some(MemShowCommands::Show {
                    all,
                    free,
                    used,
                    available,
                }) => {
                    if *all {
                        let total_memory = sys.total_memory();
                        println!("Total Memory: {} GB", bytes_to_gb(total_memory));
                    }
                    if *free {
                        let free_memory = sys.free_memory();
                        println!("Free Memory: {} GB", bytes_to_gb(free_memory));
                    }
                    if *used {
                        let used_memory = sys.used_memory();
                        println!("Used Memory: {} GB", bytes_to_gb(used_memory));
                    }
                    if *available {
                        let available_memory = sys.available_memory();
                        println!("Available Memory: {} GB", bytes_to_gb(available_memory));
                    }
                }
                None => {
                    println!("No action specified for Mem command");
                }
            }
        }
        Some(Commands::Ls { action }) => match fs::read_dir(".") {
            Ok(entries) => {
                let mut builder = Builder::default();
                let mut dir_files: Vec<String> = Vec::new();
                for entry in entries {
                    if let Ok(entry) = entry {
                        if *action {
                            let path_string = entry.path().to_string_lossy().to_string();
                            dir_files.push(path_string);
                        } else {
                            dir_files.push(entry.file_name().to_string_lossy().to_string());
                        }
                    }
                }
                builder.push_record(dir_files);
                let mut table = builder.build();
                table.with(Style::ascii_rounded());
                println!("{}", table.to_string());
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        },
        None => {
            println!("No subcommand was used");
        }
    }
}

fn bytes_to_gb(bytes: u64) -> f64 {
    let gb = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    format!("{:.2}", gb).parse().unwrap_or(0.0)
}
