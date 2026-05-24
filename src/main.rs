#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod archive;
mod compression;
mod extraction;
mod fs;
mod ui;

use ui::app::LoserArApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "LoserAR - Modern Archive Manager",
        options,
        Box::new(|cc| {
            // Apply custom dark theme
            ui::theme::setup_custom_fonts(&cc.egui_ctx);
            ui::theme::setup_dark_theme(&cc.egui_ctx);
            Box::new(LoserArApp::new(cc))
        }),
    )
}
