mod libs;
use clap::{Parser, Subcommand};
use libs::{data_controller, logger_control, win_api};
use std::{
    env::consts::OS,
    fs::{self, File},
};
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
    CPU {
        #[command(subcommand)]
        action: Option<CPUCommands>,
    },
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
enum CPUCommands {
    #[command(about = "Show CPU information", long_about = "Show CPU information")]
    Show {
        #[arg(short, long, help = "Show CPU information")]
        all: bool,
        #[arg(short, long, help = "Show CPU usage information")]
        usage: bool,
        #[arg(short, long, help = "Show CPU temperature information")]
        frequency: bool,
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
        #[arg(short = 'v', long, help = "Show available memory information")]
        available: bool,
    },
}
fn main() {
    let args = Args::parse();

    libs::logger_control::initialize();

    logger_control::log("Starting program", logger_control::LogLevel::INFO);

    data_controller::init_settings();

    match OS {
        "windows" => {
            windows_cmd(args);
            logger_control::log("Program finished", logger_control::LogLevel::INFO);
        }
        _ => {
            println!("Unsupported OS");
            logger_control::log("Unsupported OS", logger_control::LogLevel::ERROR);
        }
    }
}

fn windows_cmd(args: Args) {
    match &args.command {
        Some(Commands::CPU { action }) => {
            let mut sys = System::new_all();
            sys.refresh_all();
            match action {
                Some(CPUCommands::Show {
                    all,
                    usage,
                    frequency,
                }) => {
                    if *all {
                        for cpu in sys.cpus() {
                            println!("{}", cpu.name());
                            logger_control::log(
                                &format!("CPU name all called {}", cpu.name()),
                                logger_control::LogLevel::INFO,
                            );
                        }
                    }
                    if *usage {
                        for cpu in sys.cpus() {
                            println!("{}", cpu.cpu_usage());
                            logger_control::log(
                                &format!("CPU usage usage called {}", cpu.cpu_usage()),
                                logger_control::LogLevel::INFO,
                            );
                        }
                    }
                    if *frequency {
                        for cpu in sys.cpus() {
                            println!("{}", cpu.frequency());
                            logger_control::log(
                                &format!("CPU frequency frequency called {}", cpu.frequency()),
                                logger_control::LogLevel::INFO,
                            );
                        }
                    }
                }
                None => {
                    println!("No action specified for CPU command");
                    logger_control::log(
                        "No action specified for CPU command",
                        logger_control::LogLevel::ERROR,
                    );
                }
            }
        }

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
                        logger_control::log(
                            &format!("Total memory all called {}", total_memory),
                            logger_control::LogLevel::INFO,
                        );
                    }
                    if *free {
                        let free_memory = sys.free_memory();
                        println!("Free Memory: {} GB", bytes_to_gb(free_memory));
                        logger_control::log(
                            &format!("Free memory free called {}", free_memory),
                            logger_control::LogLevel::INFO,
                        );
                    }
                    if *used {
                        let used_memory = sys.used_memory();
                        println!("Used Memory: {} GB", bytes_to_gb(used_memory));
                        logger_control::log(
                            &format!("Used memory used called {}", used_memory),
                            logger_control::LogLevel::INFO,
                        );
                    }
                    if *available {
                        let available_memory = sys.available_memory();
                        println!("Available Memory: {} GB", bytes_to_gb(available_memory));
                        logger_control::log(
                            &format!("Available memory available called {}", available_memory),
                            logger_control::LogLevel::INFO,
                        );
                    }
                }
                None => {
                    println!("No action specified for Mem command");
                    logger_control::log(
                        "No action specified for Mem command",
                        logger_control::LogLevel::ERROR,
                    );
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
                logger_control::log("Ls command called", logger_control::LogLevel::INFO);
            }
            Err(e) => {
                println!("Error: {}", e);
                logger_control::log(&format!("Error: {}", e), logger_control::LogLevel::ERROR);
            }
        },
        None => {
            println!("No subcommand was used");
            logger_control::log("No subcommand was used", logger_control::LogLevel::ERROR);
        }
    }
}

fn bytes_to_gb(bytes: u64) -> f64 {
    let gb = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    format!("{:.2}", gb).parse().unwrap_or(0.0)
}
