use egui::{Context, Window, TextEdit};
use crate::ui::app::LoserArApp;
use crate::compression::Progress;
use std::path::PathBuf;

pub fn render(app: &mut LoserArApp, ctx: &Context) {
    if !app.show_compress_dialog {
        return;
    }

    let mut open = app.show_compress_dialog;
    
    Window::new("Compress Files")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.label(format!("Compressing {} items", app.selected_paths.len()));
            ui.add_space(8.0);
            
            // Just a basic default name based on first file
            let mut archive_name = if let Some(first) = app.selected_paths.first() {
                format!("{}.zip", first.file_stem().unwrap_or_default().to_string_lossy())
            } else {
                "archive.zip".to_string()
            };
            
            ui.horizontal(|ui| {
                ui.label("Archive Name:");
                ui.add(TextEdit::singleline(&mut archive_name).desired_width(200.0));
            });
            
            ui.add_space(16.0);
            
            ui.horizontal(|ui| {
                if ui.button("Compress").clicked() {
                    let dest_path = app.current_path.join(archive_name);
                    let sources = app.selected_paths.clone();
                    
                    let progress = Progress::new();
                    app.active_progress = Some(progress.clone());
                    
                    std::thread::spawn(move || {
                        if let Err(e) = crate::compression::compress_paths(sources, dest_path, 6, progress.clone()) {
                            progress.set_state(crate::compression::ProgressState::Error(e.to_string()));
                        }
                    });
                    
                    app.show_compress_dialog = false;
                }
                
                if ui.button("Cancel").clicked() {
                    if let Some(progress) = &app.active_progress {
                        progress.cancel();
                    }
                    app.show_compress_dialog = false;
                }
            });
        });
        
    app.show_compress_dialog = app.show_compress_dialog && open;
}
