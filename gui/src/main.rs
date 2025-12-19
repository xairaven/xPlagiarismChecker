// Hide console window on Windows in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::config::Config;
use crate::logs::Logger;
use crate::ui::Ui;

// Defining folder with locales. Path: crate/locales
rust_i18n::i18n!("locales", fallback = "en");

const PROJECT_TITLE: &str = "xPlagiarismChecker";

fn main() {
    let config = Config::from_file().unwrap_or_else(|error| {
        Ui::native_panic_message(error);
        std::process::exit(1);
    });

    rust_i18n::set_locale(&config.language.code());

    Logger::from_config(&config)
        .setup()
        .unwrap_or_else(|error| {
            Ui::native_panic_message(error);
            std::process::exit(1);
        });

    Ui::default().start(config).unwrap_or_else(|error| {
        Ui::native_panic_message(error);
        std::process::exit(1);
    });
}

mod config;
mod context;
mod errors;
mod io;
mod localization;
mod logs;
mod ui;
