#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dioxus::prelude::LaunchBuilder;
use dioxus_desktop::{tao::window::Icon, Config, WindowBuilder};
use std::{env::current_dir, path::PathBuf};

mod app;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let icon = current_dir()?
        .join("icons/favicon.ico")
        .canonicalize()
        .ok()
        .and_then(|p| load_icon(p).ok());

    LaunchBuilder::desktop()
        .with_cfg(
            Config::new().with_window(
                WindowBuilder::new()
                    .with_window_icon(icon)
            )
        )
        .launch(app::App);

    Ok(())
}

fn load_icon(path: PathBuf) -> Result<Icon, Box<dyn std::error::Error>> {
    let img = image::open(path)?.into_rgba8();
    let (width, height) = img.dimensions();
    let raw_data = img.into_raw();
    Ok(Icon::from_rgba(raw_data, width, height)?)
}
