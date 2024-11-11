use clap::{Parser, Subcommand};
use std::env::consts::OS;
use sysinfo::{Components, Disks, Networks, System};

#[derive(Parser)]
#[command(version, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand)]
enum Commands {
    Mem {
        #[command(subcommand)]
        action: Option<MemShowCommands>,
    },
}

#[derive(Subcommand)]
enum MemShowCommands {
    Show {
        #[arg(short, long)]
        all: bool,
        #[arg(short, long)]
        free: bool,
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
                Some(MemShowCommands::Show { all, free }) => {
                    if *all {
                        let total_memory = sys.total_memory();
                        println!("Total Memory: {} GB", bytes_to_gb(total_memory));
                    }
                    if *free {
                        let free_memory = sys.free_memory();
                        println!("Free Memory: {} GB", bytes_to_gb(free_memory));
                    }
                }
                None => {
                    println!("No action specified for Mem command");
                }
            }
        }
        None => {
            println!("No subcommand was used");
        }
    }
}

fn bytes_to_gb(bytes: u64) -> f64 {
    let gb = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    format!("{:.2}", gb).parse().unwrap_or(0.0)
}
