// Hide console window on Windows in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::config::Config;

// Defining folder with locales. Path: crate/locales
rust_i18n::i18n!("locales", fallback = "en");

fn main() {
    let config = Config::from_file().unwrap_or_else(|error| {
        eprintln!("Config initialization failed: {}", error);
        std::process::exit(1);
    });

    rust_i18n::set_locale(&config.language.code());
}

mod config;
mod errors;
mod io;
mod localization;
mod logs;
mod ui;
