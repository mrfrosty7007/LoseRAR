use egui::{Context, SidePanel, RichText, ScrollArea};
use crate::ui::app::LoserArApp;
use std::path::PathBuf;

pub fn render(app: &mut LoserArApp, ctx: &Context) {
    SidePanel::left("sidebar_panel")
        .resizable(true)
        .default_width(200.0)
        .show(ctx, |ui| {
            ui.heading("Locations");
            ui.add_space(8.0);

            ScrollArea::vertical().show(ui, |ui| {
                let drives = crate::fs::list_drives();
                for drive in drives {
                    let drive_name = drive.to_string_lossy().to_string();
                    if ui.selectable_label(app.current_path == drive, format!("💽 {}", drive_name)).clicked() {
                        app.current_path = drive;
                        app.refresh_dir();
                    }
                }

                ui.add_space(16.0);
                ui.heading("Quick Access");
                
                let dirs = vec![
                    ("Desktop", dirs::desktop_dir()),
                    ("Documents", dirs::document_dir()),
                    ("Downloads", dirs::download_dir()),
                    ("Pictures", dirs::picture_dir()),
                ];

                for (name, path_opt) in dirs {
                    if let Some(path) = path_opt {
                        if ui.selectable_label(app.current_path == path, format!("📁 {}", name)).clicked() {
                            app.current_path = path;
                            app.refresh_dir();
                        }
                    }
                }
            });
        });
}
