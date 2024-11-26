mod libs;
use clap::{ArgAction, Parser, Subcommand};
use libs::{data_controller, logger_control, win_api};
use std::{
    collections::HashMap,
    env::consts::OS,
    fs::{self, File},
    io::{self, Write},
};
use sysinfo::{Components, Disks, Networks, System};
use tabled::{builder::Builder, settings::Style};

// プリプロセッサー
const VERISON: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(version, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand)]
enum Commands {
    #[command(
        about = "Show command line information",
        long_about = "Show command line information"
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
    Browser {
        #[command(subcommand)]
        action: Option<BrowserCommands>,
    },
    Open {
        #[command(subcommand)]
        action: Option<OpenCommands>,
    },
    Remove {
        #[arg(short, long, help = "Remove a file")]
        remove: Option<String>,
    },
    Update {
        #[arg(short, long, help = "Update the program")]
        update: bool,
    },
    explorer {
        #[arg(short, long, help = "Open explorer")]
        reflesh: bool,
    },

    Version,
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
        #[arg(long, help = "all pid")]
        all_pid: bool,
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

#[derive(Subcommand)]
enum BrowserCommands {
    #[command(
        about = "Show browser information",
        long_about = "Show browser information"
    )]
    Show {
        #[arg(short, long, help = "Show browser information")]
        all: bool,
        #[arg(long, help = "Set Browser : Example: --set c:/path/to/browser.exe")]
        set: Option<String>,
        #[arg(long, help = "reset Browser")]
        reset: bool,
        #[arg(long, help = "Set Search Engine")]
        set_search: bool,
    },
    #[command(about = "Search in browser", long_about = "Search in browser")]
    Search {
        #[arg(value_name = "QUERY")]
        query: String,
    },
    #[command(about = "Web Favorite", long_about = "Web Favorite")]
    Favorite {
        #[arg(long, short, help = "Add a favorite URL")]
        add_favorite: bool,
        #[arg(long, short, help = "Remove a favorite URL")]
        remove_favorite: bool,
        #[arg(long, short, help = "List all favorite URLs")]
        list_favorite: bool,
        #[arg(long, short, help = "Open a favorite URL")]
        open_favorite: Option<String>,
    },
}

#[derive(Subcommand)]

enum OpenCommands {
    #[command(about = "Open a file and app", long_about = "Open a file file and app")]
    Appdata {
        #[arg(long, help = "Open Appdata")]
        user: Option<String>,
    },
    Local,
    TaskManager,
    There,
    AllSid,
}
fn main() {
    let args = Args::parse();

    libs::logger_control::initialize();

    logger_control::log("Starting program", logger_control::LogLevel::INFO);

    data_controller::init_settings();

    data_controller::null_search_settings();

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
        // cpu command
        Some(Commands::CPU { action }) => {
            let mut sys = System::new_all();
            sys.refresh_all();
            match action {
                Some(CPUCommands::Show {
                    all,
                    usage,
                    frequency,
                    all_pid,
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
                    if *all_pid {
                        win_api::show_all_pid();
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

        // mem command
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

        // ls command
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

        // browser command
        Some(Commands::Browser { action }) => {
            let settings = data_controller::read_settings();
            match action {
                Some(BrowserCommands::Show {
                    all,
                    set,
                    reset,
                    set_search,
                }) => {
                    if *all {
                        println!("Browser: {}", settings.browser);
                        logger_control::log(
                            &format!("Browser all called {}", settings.browser),
                            logger_control::LogLevel::INFO,
                        );
                    }

                    if let Some(set) = set {
                        let new_settings = data_controller::Settings {
                            version: settings.version.clone(),
                            browser: set.to_string(),
                            web_search: settings.web_search.clone(),
                        };
                        data_controller::write_settings(new_settings);
                        logger_control::log(
                            &format!("Browser set set called {}", set),
                            logger_control::LogLevel::INFO,
                        );
                    }

                    if *reset {
                        let new_settings = data_controller::Settings {
                            version: settings.version.clone(),
                            browser: "Default".to_string(),
                            web_search: settings.web_search.clone(),
                        };
                        data_controller::write_settings(new_settings);
                        logger_control::log(
                            &format!("Browser reset reset called"),
                            logger_control::LogLevel::INFO,
                        );
                    }

                    if *set_search {
                        println!("Please enter the search engine you would like to use");
                        println!("Options: 1:DuckDuckGo 2:Google");
                        let mut input = String::new();

                        print!("Enter your choice: ");
                        io::stdout().flush().unwrap();

                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read line");

                        let input = input.trim();

                        let new_settings = data_controller::Settings {
                            version: settings.version.clone(),
                            browser: settings.browser.clone(),
                            web_search: match input {
                                "1" => "DuckDuckGo".to_string(),
                                "2" => "Google".to_string(),
                                _ => {
                                    println!("Invalid input, defaulting to DuckDuckGo");
                                    "DuckDuckGo".to_string()
                                }
                            },
                        };

                        data_controller::write_settings(new_settings);
                        logger_control::log(
                            &format!("Browser set_search set_search called {}", input),
                            logger_control::LogLevel::INFO,
                        );
                    }
                }
                None => {
                    println!("No action specified for Browser command");
                    logger_control::log(
                        "No action specified for Browser command",
                        logger_control::LogLevel::ERROR,
                    );
                }

                Some(BrowserCommands::Favorite {
                    add_favorite,
                    remove_favorite,
                    list_favorite,
                    open_favorite,
                }) => {
                    if *add_favorite {
                        data_controller::init_favorites();

                        let mut favorites = data_controller::read_favorites();
                        println!("Please enter the name of the favorite URL");
                        let mut name = String::new();
                        print!("Enter the name: ");
                        io::stdout().flush().unwrap();
                        io::stdin()
                            .read_line(&mut name)
                            .expect("Failed to read line");
                        let name = name.trim();

                        println!("Please enter the URL");
                        let mut url = String::new();
                        print!("Enter the URL: ");
                        io::stdout().flush().unwrap();
                        io::stdin()
                            .read_line(&mut url)
                            .expect("Failed to read line");
                        let url = url.trim();

                        favorites
                            .favorites
                            .insert(name.to_string(), url.to_string());
                        data_controller::write_favorites(favorites);
                        logger_control::log(
                            &format!("Browser favorite favorite called {}", name),
                            logger_control::LogLevel::INFO,
                        );
                    }
                    if *remove_favorite {
                        let favorites = data_controller::read_favorites();
                        let mut favorites_map = favorites.favorites;
                        let mut keys: Vec<String> = Vec::new();
                        for key in favorites_map.keys() {
                            keys.push(key.to_string());
                        }
                        let mut builder = Builder::default();
                        builder.push_record(keys);
                        let mut table = builder.build();
                        table.with(Style::ascii_rounded());
                        println!("{}", table.to_string());

                        println!(
                            "Please enter the name of the favorite URL you would like to remove"
                        );
                        let mut name = String::new();
                        print!("Enter the name: ");
                        io::stdout().flush().unwrap();
                        io::stdin()
                            .read_line(&mut name)
                            .expect("Failed to read line");
                        let name = name.trim();

                        favorites_map.remove(name);
                        let new_favorites = data_controller::Favorites {
                            favorites: favorites_map,
                        };
                        data_controller::write_favorites(new_favorites);
                        logger_control::log(
                            &format!("Browser favorite remove_favorite called {}", name),
                            logger_control::LogLevel::INFO,
                        );
                    }

                    if *list_favorite {
                        let favorites = data_controller::read_favorites();
                        let favorites_map = favorites.favorites;
                        let mut keys: Vec<String> = Vec::new();
                        for key in favorites_map.keys() {
                            keys.push(key.to_string());
                        }
                        let mut builder = Builder::default();
                        builder.push_record(keys);
                        let mut table = builder.build();
                        table.with(Style::ascii_rounded());
                        println!("{}", table.to_string());
                        logger_control::log(
                            "Browser favorite list_favorite called",
                            logger_control::LogLevel::INFO,
                        );
                    }

                    if let Some(oepn_favorite) = open_favorite {
                        let favorites = data_controller::read_favorites();
                        let favorites_map = favorites.favorites;
                        if let Some(favorite_url) = favorites_map.get(oepn_favorite) {
                            libs::browser_controller::search_in_browser(
                                &favorite_url,
                                &settings.web_search,
                            );
                            logger_control::log(
                                &format!(
                                    "Browser favorite open_favorite open_favorite called {}",
                                    oepn_favorite
                                ),
                                logger_control::LogLevel::INFO,
                            );
                        } else {
                            println!("Favorite URL not found");
                            logger_control::log(
                                "Favorite URL not found",
                                logger_control::LogLevel::ERROR,
                            );
                        }
                    }
                }

                Some(BrowserCommands::Search { query }) => {
                    let settings = data_controller::read_settings();
                    libs::browser_controller::search_in_browser(query, &settings.web_search);
                    logger_control::log(
                        &format!("Browser search search called {}", query),
                        logger_control::LogLevel::INFO,
                    );
                }
            }
        }

        // open command
        Some(Commands::Open { action }) => {
            let appdata_content = win_api::get_appdata();
            match action {
                Some(OpenCommands::Appdata { user }) => {
                    if let Some(appdata_content) = appdata_content {
                        if user.clone().unwrap().len() > 0 {
                            if let Err(e) = win_api::open_explorer(appdata_content) {
                                println!("Failed to open Appdata: {}", e);
                                logger_control::log(
                                    &format!("Failed to open Appdata: {}", e),
                                    logger_control::LogLevel::ERROR,
                                );
                            }
                        } else {
                            if let Err(e) = win_api::open_explorer(appdata_content.join("Local")) {
                                println!("Failed to open Local Appdata: {}", e);
                                logger_control::log(
                                    &format!("Failed to open Local Appdata: {}", e),
                                    logger_control::LogLevel::ERROR,
                                );
                            }
                        }
                    } else {
                        println!("Failed to get Appdata directory");
                        logger_control::log(
                            "Failed to get Appdata directory",
                            logger_control::LogLevel::ERROR,
                        );
                    }
                }

                Some(OpenCommands::Local) => {
                    let local_appdata = win_api::get_local_appdata();
                    if let Some(local_appdata) = local_appdata {
                        if let Err(e) = win_api::open_explorer(local_appdata) {
                            println!("Failed to open Local Appdata: {}", e);
                            logger_control::log(
                                &format!("Failed to open Local Appdata: {}", e),
                                logger_control::LogLevel::ERROR,
                            );
                        }
                    } else {
                        println!("Failed to get Appdata directory");
                        logger_control::log(
                            "Failed to get Appdata directory",
                            logger_control::LogLevel::ERROR,
                        );
                    }
                }

                Some(OpenCommands::TaskManager) => {
                    if let Err(e) = win_api::open_task_manager() {
                        println!("Failed to open Task Manager: {}", e);
                        logger_control::log(
                            &format!("Failed to open Task Manager: {}", e),
                            logger_control::LogLevel::ERROR,
                        );
                    }
                }

                Some(OpenCommands::There) => {
                    if let Err(e) = win_api::open_explorer(".") {
                        println!("Failed to open current directory: {}", e);
                        logger_control::log(
                            &format!("Failed to open current directory: {}", e),
                            logger_control::LogLevel::ERROR,
                        );
                    }
                }

                Some(OpenCommands::AllSid) => match win_api::get_all_user_sids() {
                    Ok(sids) => {
                        for sid in sids {
                            println!("{}", sid);
                        }
                    }
                    Err(e) => {
                        println!("Failed to get all SIDs: {}", e);
                        logger_control::log(
                            &format!("Failed to get all SIDs: {}", e),
                            logger_control::LogLevel::ERROR,
                        );
                    }
                },

                None => {
                    println!("No action specified for Open command");
                    logger_control::log(
                        "No action specified for Open command",
                        logger_control::LogLevel::ERROR,
                    );
                }
            }
        }

        // remove command
        Some(Commands::Remove { remove }) => {
            if let Some(remove) = remove {
                if let Err(e) = fs::remove_file(remove) {
                    println!("Failed to remove file: {}", e);
                    logger_control::log(
                        &format!("Failed to remove file: {}", e),
                        logger_control::LogLevel::ERROR,
                    );
                }
            }
        }

        // update command
        Some(Commands::Update { update }) => {
            if *update {
                // use velopack as library
                print!("Updating program");
            }
        }

        // version command
        Some(Commands::Version) => {
            println!("Version: {}", VERISON);
            logger_control::log(
                &format!("Version version called {}", VERISON),
                logger_control::LogLevel::INFO,
            );
        }

        // explorer command
        Some(Commands::explorer { reflesh }) => {
            if *reflesh {
                let _ = win_api::refresh_exprorer();
                logger_control::log("Explorer reflesh called", logger_control::LogLevel::INFO);
            }
        }
    }
}

fn bytes_to_gb(bytes: u64) -> f64 {
    let gb = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    format!("{:.2}", gb).parse().unwrap_or(0.0)
}
