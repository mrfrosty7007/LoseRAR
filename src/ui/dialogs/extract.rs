use egui::{Context, Window, TextEdit};
use crate::ui::app::LoserArApp;
use crate::compression::Progress;

pub fn render(app: &mut LoserArApp, ctx: &Context) {
    if !app.show_extract_dialog {
        return;
    }

    let mut open = app.show_extract_dialog;
    
    Window::new("Extract Files")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            let archive_path = app.selected_paths.first().cloned().unwrap_or_default();
            ui.label(format!("Extracting: {}", archive_path.file_name().unwrap_or_default().to_string_lossy()));
            ui.add_space(8.0);
            
            let mut extract_dir = archive_path.parent().unwrap_or(&app.current_path).to_path_buf();
            let mut extract_dir_str = extract_dir.to_string_lossy().to_string();
            
            ui.horizontal(|ui| {
                ui.label("Destination:");
                ui.add(TextEdit::singleline(&mut extract_dir_str).desired_width(200.0));
                if ui.button("Browse...").clicked() {
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                        extract_dir_str = folder.to_string_lossy().to_string();
                    }
                }
            });
            
            ui.add_space(16.0);
            
            ui.horizontal(|ui| {
                if ui.button("Extract").clicked() {
                    let dest_path = std::path::PathBuf::from(extract_dir_str);
                    
                    let progress = Progress::new();
                    app.active_progress = Some(progress.clone());
                    
                    std::thread::spawn(move || {
                        if let Err(e) = crate::extraction::extract_archive(archive_path, dest_path, progress.clone()) {
                            progress.set_state(crate::compression::ProgressState::Error(e.to_string()));
                        }
                    });
                    
                    app.show_extract_dialog = false;
                }
                
                if ui.button("Cancel").clicked() {
                    app.show_extract_dialog = false;
                }
            });
        });
        
    app.show_extract_dialog = open;
}
