#![windows_subsystem = "windows"]

use dioxus::prelude::*;
mod app;

fn main() {
    launch(app::App);
}
