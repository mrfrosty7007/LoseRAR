use egui::{Align, Context, Layout, TopBottomPanel, Button, RichText};
use crate::ui::app::LoserArApp;

pub fn render(app: &mut LoserArApp, ctx: &Context) {
    TopBottomPanel::top("toolbar_panel")
        .exact_height(48.0)
        .show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.add_space(8.0);
                
                if ui.add_sized([100.0, 32.0], Button::new(RichText::new("📦 Compress").size(16.0))).clicked() {
                    if !app.selected_paths.is_empty() {
                        app.show_compress_dialog = true;
                    } else if let Some(path) = rfd::FileDialog::new().pick_files() {
                        app.selected_paths = path;
                        app.show_compress_dialog = true;
                    }
                }
                
                ui.add_space(8.0);
                
                if ui.add_sized([100.0, 32.0], Button::new(RichText::new("📂 Extract").size(16.0))).clicked() {
                    if let Some(file) = app.selected_paths.first() {
                        if file.extension().unwrap_or_default() == "zip" {
                            app.show_extract_dialog = true;
                        }
                    } else if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Archive", &["zip"])
                        .pick_file() {
                        app.selected_paths = vec![path];
                        app.show_extract_dialog = true;
                    }
                }
            });
        });
}
