use std::{env::consts::OS, fs, fs::OpenOptions, io::Write, path::PathBuf};

use once_cell::sync::OnceCell;

use dirs;

static LOG_TIME: OnceCell<PathBuf> = OnceCell::new();

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    INFO,
    ERROR,
    WARNING,
    DEBUG,
    CRITICAL,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::INFO => "INFO",
            LogLevel::ERROR => "ERROR",
            LogLevel::WARNING => "WARNING",
            LogLevel::DEBUG => "DEBUG",
            LogLevel::CRITICAL => "CRITICAL",
        }
    }
}

pub fn initialize() {
    let time = chrono::Local::now().format("%Y-%m-%d").to_string();

    match OS {
        "windows" => {
            let local_app_data =
                dirs::data_local_dir().expect("Failed to get local app data directory");
            let log_dir = local_app_data.join("johma_windows_enhanced").join("logs");
            if !log_dir.exists() {
                fs::create_dir_all(&log_dir).expect("Failed to create log directory");
            }

            let log_file = log_dir.join(format!("{}.log", time));
            LOG_TIME.set(log_file).unwrap();
            if !LOG_TIME.get().unwrap().exists() {
                fs::File::create(LOG_TIME.get().unwrap()).unwrap();
            }
        }

        _ => {
            println!("logging not implemented for this OS");
        }
    }
}

pub fn log(message: &str, level: LogLevel) {
    if let Some(log_path) = LOG_TIME.get() {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_message = format!("{}: {} - {}\n", level.as_str(), timestamp, message);

        match OpenOptions::new().create(true).append(true).open(log_path) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(log_message.as_bytes()) {
                    eprintln!("Failed to write to log file: {}", e);
                }
            }
            Err(e) => eprintln!("Failed to open log file: {}", e),
        }
    } else {
        eprintln!("Log file path not initialized");
    }
}
