// Hide console window on Windows in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::files::AppFiles;
use crate::logs::Logger;
use crate::ui::Ui;

// Defining folder with locales. Path: crate/locales
rust_i18n::i18n!("locales", fallback = "en");

const PROJECT_TITLE: &str = "xPlagiarismChecker";

fn main() {
    let app_files = AppFiles::load().unwrap_or_else(|error| {
        Ui::native_panic_message(error);
        std::process::exit(1);
    });

    rust_i18n::set_locale(&app_files.config.language.code());

    Logger::from_config(&app_files.config)
        .setup()
        .unwrap_or_else(|error| {
            Ui::native_panic_message(error);
            std::process::exit(1);
        });

    Ui::default().start(app_files).unwrap_or_else(|error| {
        Ui::native_panic_message(error);
        std::process::exit(1);
    });
}

mod context;
mod errors;
mod files;
mod localization;
mod logs;
mod session;
mod ui;
