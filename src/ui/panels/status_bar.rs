use egui::{Context, TopBottomPanel, ProgressBar};
use crate::ui::app::LoserArApp;
use crate::compression::ProgressState;

pub fn render(app: &mut LoserArApp, ctx: &Context) {
    TopBottomPanel::bottom("status_bar")
        .exact_height(32.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("{} items selected", app.selected_paths.len()));
                
                if let Some(progress) = &app.active_progress {
                    let state = progress.get_state();
                    match state {
                        ProgressState::Idle => {}
                        ProgressState::Scanning => {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.spinner();
                                ui.label("Scanning files...");
                            });
                        }
                        ProgressState::Working { current_file, files_processed, total_files, bytes_processed, total_bytes } => {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                let pct = if total_bytes > 0 {
                                    bytes_processed as f32 / total_bytes as f32
                                } else {
                                    0.0
                                };
                                ui.add(ProgressBar::new(pct).show_percentage());
                                ui.label(format!("Processing: {} ({}/{})", current_file, files_processed, total_files));
                            });
                            // Request repaint to keep animation smooth
                            ctx.request_repaint();
                        }
                        ProgressState::Finished { success, message } => {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("Clear").clicked() {
                                    app.active_progress = None;
                                    app.refresh_dir();
                                }
                                if success {
                                    ui.label(egui::RichText::new(message).color(egui::Color32::GREEN));
                                } else {
                                    ui.label(egui::RichText::new(message).color(egui::Color32::RED));
                                }
                            });
                        }
                        ProgressState::Error(err) => {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("Clear").clicked() {
                                    app.active_progress = None;
                                }
                                ui.label(egui::RichText::new(format!("Error: {}", err)).color(egui::Color32::RED));
                            });
                        }
                    }
                }
            });
        });
}
