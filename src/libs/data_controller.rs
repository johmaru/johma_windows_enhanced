use std::collections::HashMap;
use std::fs;

use dirs;
use serde::Deserialize;
use serde::Serialize;

use crate::VERISON;

use super::logger_control;

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    pub version: String,
    pub browser: String,
    pub web_search: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Favorites {
    pub favorites: HashMap<String, String>,
}

pub fn read_favorites() -> Favorites {
    let local_data = dirs::data_local_dir().expect("Failed to get local app data directory");

    let favorites_loc = local_data
        .join("johma_windows_enhanced")
        .join("favorites.toml");

    let contents =
        fs::read_to_string(favorites_loc).expect("Something went wrong reading the file");

    toml::from_str(&contents).expect("Failed to parse favorites file")
}

pub fn init_favorites() {
    let local_data = dirs::data_local_dir().expect("Failed to get local app data directory");

    let favorites_loc = local_data
        .join("johma_windows_enhanced")
        .join("favorites.toml");

    if !favorites_loc.exists() {
        let favorites = Favorites {
            favorites: HashMap::new(),
        };

        let toml = toml::to_string(&favorites).expect("Failed to serialize favorites");

        fs::write(favorites_loc, toml).expect("Failed to write favorites file");

        logger_control::log("Create new favorites file", logger_control::LogLevel::INFO);
    }

    logger_control::log(
        "Favorites file already exists",
        logger_control::LogLevel::INFO,
    );
}

pub fn write_favorites(favorites: Favorites) {
    let local_data = dirs::data_local_dir().expect("Failed to get local app data directory");

    let favorites_loc = local_data
        .join("johma_windows_enhanced")
        .join("favorites.toml");

    let toml = toml::to_string(&favorites).expect("Failed to serialize favorites");

    fs::write(favorites_loc, toml).expect("Failed to write favorites file");

    logger_control::log("Write favorites file", logger_control::LogLevel::INFO);
}

pub fn read_settings() -> Settings {
    let local_data = dirs::data_local_dir().expect("Failed to get local app data directory");

    let setting_loc = local_data
        .join("johma_windows_enhanced")
        .join("settings.toml");

    let contents = fs::read_to_string(setting_loc).expect("Something went wrong reading the file");

    toml::from_str(&contents).expect("Failed to parse settings file")
}

pub fn null_search_settings() {
    let settings = read_settings();

    if settings.version == "" {
        let new_settings = Settings {
            version: VERISON.to_string(),
            browser: settings.browser,
            web_search: settings.web_search,
        };

        write_settings(new_settings);
    } else if settings.browser == "" {
        let new_settings = Settings {
            version: settings.version,
            browser: "Default".to_string(),
            web_search: settings.web_search,
        };
        write_settings(new_settings);
    } else if settings.web_search == "" {
        let new_settings = Settings {
            version: settings.version,
            browser: settings.browser,
            web_search: "DuckDuckGo".to_string(),
        };
        write_settings(new_settings);
    }
}

pub fn init_settings() {
    let local_data = dirs::data_local_dir().expect("Failed to get local app data directory");

    let setting_loc = local_data
        .join("johma_windows_enhanced")
        .join("settings.toml");

    if !setting_loc.exists() {
        let settings = Settings {
            version: VERISON.to_string(),
            browser: "Default".to_string(),
            web_search: "DuckDuckGo".to_string(),
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
