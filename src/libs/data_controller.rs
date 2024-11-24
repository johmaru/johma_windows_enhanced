use std::fs;

use dirs;
use serde::Deserialize;
use serde::Serialize;

use super::logger_control;

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    pub version: String,
    pub browser: String,
}

pub fn read_settings() -> Settings {
    let local_data = dirs::data_local_dir().expect("Failed to get local app data directory");

    let setting_loc = local_data
        .join("johma_windows_enhanced")
        .join("settings.toml");

    let contents = fs::read_to_string(setting_loc).expect("Something went wrong reading the file");

    toml::from_str(&contents).expect("Failed to parse settings file")
}

pub fn init_settings() {
    let local_data = dirs::data_local_dir().expect("Failed to get local app data directory");

    let setting_loc = local_data
        .join("johma_windows_enhanced")
        .join("settings.toml");

    if !setting_loc.exists() {
        let settings = Settings {
            version: "0.1.0".to_string(),
            browser: "Default".to_string(),
        };

        let toml = toml::to_string(&settings).expect("Failed to serialize settings");

        fs::write(setting_loc, toml).expect("Failed to write settings file");

        logger_control::log("Create new settings file", logger_control::LogLevel::INFO);
    }

    logger_control::log(
        "Settings file already exists",
        logger_control::LogLevel::INFO,
    );
}

pub fn write_settings(settings: Settings) {
    let local_data = dirs::data_local_dir().expect("Failed to get local app data directory");

    let setting_loc = local_data
        .join("johma_windows_enhanced")
        .join("settings.toml");

    let toml = toml::to_string(&settings).expect("Failed to serialize settings");

    fs::write(setting_loc, toml).expect("Failed to write settings file");

    logger_control::log("Write settings file", logger_control::LogLevel::INFO);
}
