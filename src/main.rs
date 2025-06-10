#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dioxus::prelude::*;
mod app;

fn main() {
    let result = std::panic::catch_unwind(|| {
        launch(app::App);
    });

    match result {
        Ok(_) => std::process::exit(0),
        Err(_) => std::process::exit(1),
    }
}
