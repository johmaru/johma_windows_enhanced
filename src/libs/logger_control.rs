use std::{env::consts::OS, fs, path::PathBuf};

use once_cell::sync::OnceCell;

use dirs;

static LOG_TIME: OnceCell<PathBuf> = OnceCell::new();

pub fn initialize() {
    let time = chrono::Local::now().format("%Y-%m-%d %H-%M-%S").to_string();

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

        _ => {}
    }
}
