#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dioxus::prelude::*;
mod app;

fn main() {
    launch(app::App);
}
