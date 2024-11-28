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
    #[command(about = "cpu information", long_about = "cpu information")]
    CPU {
        #[command(subcommand)]
        action: Option<CPUCommands>,
    },
    #[command(about = "memory information", long_about = "memory information")]
    Mem {
        #[command(subcommand)]
        action: Option<MemShowCommands>,
    },
    #[command(
        about = "List files in the current directory",
        long_about = "List files in the current directory"
    )]
    Ls {
        #[arg(long)]
        action: bool,
    },
    #[command(about = "Browser information", long_about = "Browser information")]
    Browser {
        #[command(subcommand)]
        action: Option<BrowserCommands>,
    },
    #[command(
        about = "Open a file or app or windows system",
        long_about = "Open a file or app or windows system"
    )]
    Open {
        #[command(subcommand)]
        action: Option<OpenCommands>,
    },
    #[command(about = "Remove a file", long_about = "Remove a file")]
    Rm {
        #[arg(short, long, help = "Remove a file")]
        remove: Option<String>,
    },
    #[command(about = "Update the program", long_about = "Update the program")]
    Update {
        #[arg(short, long, help = "Update the program")]
        update: bool,
    },
    #[command(
        about = "control windows file explorer",
        long_about = "control windows file explorer"
    )]
    Expl {
        #[arg(short, long, help = "reflesh explorer")]
        reflesh: bool,
    },
    #[command(about = "Process control", long_about = "Process control")]
    Proc {
        #[command(subcommand)]
        action: Option<ProcCommands>,
    },
    #[command(about = "Launcher Control", long_about = "Launcher Control")]
    Lc {
        #[command(subcommand)]
        action: Option<LcCommands>,
    },

    #[command(about = "Show version", long_about = "Show version")]
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
    Fav {
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
    #[command(about = "Open Local Appdata", long_about = "Open Local Appdata")]
    Local,
    #[command(about = "Open LocalLow", long_about = "Open LocalLow")]
    LocalLow,
    #[command(about = "Open Roaming", long_about = "Open Roaming")]
    Roaming,
    #[command(about = "Open Task Manager", long_about = "Open Task Manager")]
    TaskM,
    #[command(about = "Open this appfolder", long_about = "Open this appfolder")]
    Johma,
    #[command(
        about = "Open Environment Variables",
        long_about = "Open Environment Variables"
    )]
    Env,
    #[command(
        about = "Open current directory",
        long_about = "Open current directory"
    )]
    There,
    #[command(about = "Open all SIDs", long_about = "Open all SIDs")]
    AllSid,
}

#[derive(Subcommand)]
enum ProcCommands {
    #[command(about = "Show all processes", long_about = "Show all processes")]
    Show {
        #[arg(short, long, help = "Show all processes")]
        all: bool,
    },
    #[command(about = "Kill a process", long_about = "Kill a process")]
    Kill {
        #[arg(short, long, help = "Kill a process")]
        pid: u32,
    },
}

#[derive(Subcommand)]
enum LcCommands {
    #[command(about = "Show all launchers", long_about = "Show all launchers")]
    Show {
        #[arg(short, long, help = "Show all launchers")]
        all: bool,
    },
    #[command(about = "Add a launcher", long_about = "Add a launcher")]
    Add,
    #[command(about = "Remove a launcher", long_about = "Remove a launcher")]
    Remove,
    #[command(about = "Run a launcher", long_about = "Run a launcher")]
    Run { name: String },
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

                Some(BrowserCommands::Fav {
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

                Some(OpenCommands::LocalLow) => {
                    let local_low = win_api::get_local_low();
                    if let Some(local_low) = local_low {
                        if let Err(e) = win_api::open_explorer(local_low) {
                            println!("Failed to open LocalLow: {}", e);
                            logger_control::log(
                                &format!("Failed to open LocalLow: {}", e),
                                logger_control::LogLevel::ERROR,
                            );
                        }
                    } else {
                        println!("Failed to get LocalLow directory");
                        logger_control::log(
                            "Failed to get LocalLow directory",
                            logger_control::LogLevel::ERROR,
                        );
                    }
                }

                Some(OpenCommands::Roaming) => {
                    let roaming = win_api::get_roaming();
                    if let Some(roaming) = roaming {
                        if let Err(e) = win_api::open_explorer(roaming) {
                            println!("Failed to open Roaming: {}", e);
                            logger_control::log(
                                &format!("Failed to open Roaming: {}", e),
                                logger_control::LogLevel::ERROR,
                            );
                        }
                    } else {
                        println!("Failed to get Roaming directory");
                        logger_control::log(
                            "Failed to get Roaming directory",
                            logger_control::LogLevel::ERROR,
                        );
                    }
                }

                Some(OpenCommands::Johma) => {
                    let app_folder = win_api::get_app_folder();
                    if let Some(app_folder) = app_folder {
                        if let Err(e) = win_api::open_explorer(app_folder) {
                            println!("Failed to open App Folder: {}", e);
                            logger_control::log(
                                &format!("Failed to open App Folder: {}", e),
                                logger_control::LogLevel::ERROR,
                            );
                        }
                    } else {
                        println!("Failed to get App Folder directory");
                        logger_control::log(
                            "Failed to get App Folder directory",
                            logger_control::LogLevel::ERROR,
                        );
                    }
                }

                Some(OpenCommands::TaskM) => {
                    if let Err(e) = win_api::open_task_manager() {
                        println!("Failed to open Task Manager: {}", e);
                        logger_control::log(
                            &format!("Failed to open Task Manager: {}", e),
                            logger_control::LogLevel::ERROR,
                        );
                    }
                }

                Some(OpenCommands::Env) => {
                    if let Err(e) = win_api::open_environment_variables_window() {
                        println!("Failed to open Environment Variables: {}", e);
                        logger_control::log(
                            &format!("Failed to open Environment Variables: {}", e),
                            logger_control::LogLevel::ERROR,
                        );
                    }
                    logger_control::log(
                        "Opened Environment Variables",
                        logger_control::LogLevel::INFO,
                    );
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
        Some(Commands::Rm { remove }) => {
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
        Some(Commands::Expl { reflesh }) => {
            if *reflesh {
                let _ = win_api::refresh_exprorer();
                logger_control::log("Explorer reflesh called", logger_control::LogLevel::INFO);
            }
        }

        // proc command
        Some(Commands::Proc { action }) => match action {
            None => {
                println!("No action specified for Proc command");
                logger_control::log(
                    "No action specified for Proc command",
                    logger_control::LogLevel::ERROR,
                );
            }
            Some(ProcCommands::Show { all }) => {
                if *all {
                    win_api::show_all_pid();
                    logger_control::log(
                        "CPU all_pid all_pid called",
                        logger_control::LogLevel::INFO,
                    );
                }
            }
            Some(ProcCommands::Kill { pid }) => {
                if let Err(e) = win_api::kill_pid(*pid) {
                    println!("Failed to kill process: {}", e);
                    logger_control::log(
                        &format!("Failed to kill process: {}", e),
                        logger_control::LogLevel::ERROR,
                    );
                }
            }
        },

        // lc command
        Some(Commands::Lc { action }) => match action {
            None => {
                println!("No action specified for Lc command");
                logger_control::log(
                    "No action specified for Lc command",
                    logger_control::LogLevel::ERROR,
                );
            }
            Some(LcCommands::Show { all }) => {
                if *all {
                    data_controller::init_launcher().expect("Failed to init launcher");

                    let launchers = data_controller::read_launcher().launchers;
                    let mut keys: Vec<String> = Vec::new();
                    for key in launchers.keys() {
                        keys.push(key.to_string());
                    }
                    let mut builder = Builder::default();
                    builder.push_record(keys);
                    let mut table = builder.build();
                    table.with(Style::ascii_rounded());
                    println!("{}", table.to_string());
                    logger_control::log(
                        "Launcher all_pid all_pid called",
                        logger_control::LogLevel::INFO,
                    );
                }
            }
            Some(LcCommands::Add) => {
                data_controller::init_launcher().expect("Failed to init launcher");
                let mut launchers = data_controller::read_launcher().launchers;

                println!("Please enter the name of the launcher");
                let mut name = String::new();
                print!("Enter the name: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");
                let name = name.trim();

                println!("Please enter the path of the launcher");
                let mut path = String::new();
                print!("Enter the path: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut path)
                    .expect("Failed to read line");
                let path = path.trim();

                match launchers.insert(name.to_string(), path.to_string()) {
                    Some(_) => {
                        println!("Launcher {} already exists", name);

                        let message = format!("Launcher {} already exists", name);
                        logger_control::log(&message.to_string(), logger_control::LogLevel::ERROR);
                    }
                    None => {
                        println!("Launcher {} added", name);
                    }
                }
                data_controller::write_launcher(launchers);
                logger_control::log(
                    &format!("Launcher add add called {}", name),
                    logger_control::LogLevel::INFO,
                );
            }
            Some(LcCommands::Remove) => {
                data_controller::init_launcher().expect("Failed to init launcher");

                let mut launchers = data_controller::read_launcher().launchers;
                println!("Please enter the name of the launcher you would like to remove");
                let mut name = String::new();
                print!("Enter the name: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");
                let name = name.trim();
                match launchers.remove(name) {
                    Some(_) => {
                        println!("Launcher removed");
                    }
                    None => {
                        println!("Launcher not found");
                        logger_control::log(
                            &format!("Launcher not found {}", name),
                            logger_control::LogLevel::ERROR,
                        );
                    }
                }
                data_controller::write_launcher(launchers);
                logger_control::log(
                    &format!("Launcher remove remove called {}", name),
                    logger_control::LogLevel::INFO,
                );
            }

            Some(LcCommands::Run { name }) => {
                data_controller::init_launcher().expect("Failed to init launcher");

                let launchers = data_controller::read_launcher().launchers;
                if let Some(launcher_path) = launchers.get(name) {
                    if let Err(e) = win_api::run_launcher(launcher_path) {
                        println!("Failed to run launcher: {}", e);
                        logger_control::log(
                            &format!("Failed to run launcher: {}", e),
                            logger_control::LogLevel::ERROR,
                        );
                    }
                } else {
                    println!("Launcher not found");
                    logger_control::log(
                        &format!("Launcher not found {}", name),
                        logger_control::LogLevel::ERROR,
                    );
                }
            }
        },
    }
}

fn bytes_to_gb(bytes: u64) -> f64 {
    let gb = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    format!("{:.2}", gb).parse().unwrap_or(0.0)
}
