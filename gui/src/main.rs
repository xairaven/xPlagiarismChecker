// Hide console window on Windows in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::config::Config;

#[macro_use]
extern crate rust_i18n;
// Defining folder with locales. Path: crate/locales
rust_i18n::i18n!("locales", fallback = "en");

fn main() {
    let config = Config::from_file().unwrap_or_else(|error| {
        eprintln!("Config initialization failed: {}", error);
        std::process::exit(1);
    });

    dbg!(config);
}

mod config;
mod errors;
mod io;
mod localization;
mod logs;
mod ui;
