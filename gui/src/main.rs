// Hide console window on Windows in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate rust_i18n;

// Defining folder with locales. Path: crate/locales
rust_i18n::i18n!("locales", fallback = "en");

fn main() {
    println!("Hello, world!");
}

mod config;
mod errors;
mod io;
mod localization;
mod logs;
mod ui;
