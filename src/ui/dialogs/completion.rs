use egui::{Context, Window, Color32, RichText, Frame, Rounding, Stroke, Vec2, Align2};
use crate::ui::app::LoserArApp;
use crate::compression::ProgressState;

pub fn render(app: &mut LoserArApp, ctx: &Context) {
    let mut show = false;
    
    if let Some(progress) = &app.active_progress {
        let state = progress.get_state();
        match state {
            ProgressState::Finished { .. } | ProgressState::Error(_) => {
                show = true;
            }
            _ => {}
        }
    }

    if !show {
        return;
    }

    // Background dimming effect
    let screen_rect = ctx.screen_rect();
    let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::PanelResizeLine, egui::Id::new("dimming_layer")));
    painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(180));

    let mut clear_progress = false;

    let frame = Frame::window(&ctx.style())
        .fill(Color32::from_rgb(30, 30, 35))
        .rounding(Rounding::same(12.0))
        .stroke(Stroke::new(1.0, Color32::from_rgb(60, 60, 70)))
        .inner_margin(24.0);

    Window::new("Completion")
        .frame(frame)
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(ctx, |ui| {
            if let Some(progress) = &app.active_progress {
                let state = progress.get_state();
                match state {
                    ProgressState::Finished {
                        success,
                        message,
                        archive_name: _,
                        file_count,
                        original_size,
                        compressed_size,
                        duration_secs,
                    } => {
                        let is_extraction = original_size == 0 && compressed_size == 0;
                        ui.vertical_centered(|ui| {
                            if success {
                                ui.label(RichText::new("✅").size(48.0).color(Color32::from_rgb(50, 200, 50)));
                                ui.add_space(8.0);
                                let title = if is_extraction { "Extraction Complete!" } else { "Compression Complete!" };
                                ui.label(RichText::new(title).size(24.0).strong().color(Color32::WHITE));
                            } else {
                                ui.label(RichText::new("❌").size(48.0).color(Color32::from_rgb(200, 50, 50)));
                                ui.add_space(8.0);
                                let title = if is_extraction { "Extraction Failed" } else { "Compression Failed" };
                                ui.label(RichText::new(title).size(24.0).strong().color(Color32::WHITE));
                            }
                            ui.add_space(8.0);
                            ui.label(RichText::new(message).size(14.0).color(Color32::from_rgb(180, 180, 190)));
                            
                            ui.add_space(24.0);
                            
                            if success {
                                egui::Grid::new("stats_grid")
                                    .num_columns(2)
                                    .spacing([40.0, 12.0])
                                    .show(ui, |ui| {
                                        ui.label(RichText::new("Files Processed:").color(Color32::from_rgb(150, 150, 160)));
                                        ui.label(RichText::new(file_count.to_string()).strong().color(Color32::WHITE));
                                        ui.end_row();

                                        if !is_extraction {
                                            let space_saved_percent = if original_size > 0 {
                                                let compressed_pct = (compressed_size as f64 / original_size as f64) * 100.0;
                                                (100.0 - compressed_pct).max(0.0)
                                            } else {
                                                0.0
                                            };
                                            let original_mb = original_size as f64 / 1_048_576.0;
                                            let compressed_mb = compressed_size as f64 / 1_048_576.0;

                                            ui.label(RichText::new("Original Size:").color(Color32::from_rgb(150, 150, 160)));
                                            ui.label(RichText::new(format!("{:.1} MB", original_mb)).strong().color(Color32::WHITE));
                                            ui.end_row();

                                            ui.label(RichText::new("Compressed Size:").color(Color32::from_rgb(150, 150, 160)));
                                            ui.label(RichText::new(format!("{:.1} MB", compressed_mb)).strong().color(Color32::WHITE));
                                            ui.end_row();

                                            ui.label(RichText::new("Space Saved:").color(Color32::from_rgb(150, 150, 160)));
                                            ui.label(RichText::new(format!("{:.1}%", space_saved_percent)).strong().color(Color32::from_rgb(100, 200, 255)));
                                            ui.end_row();
                                        }

                                        ui.label(RichText::new("Time Taken:").color(Color32::from_rgb(150, 150, 160)));
                                        ui.label(RichText::new(format!("{:.1}s", duration_secs)).strong().color(Color32::WHITE));
                                        ui.end_row();
                                    });
                            }
                            
                            ui.add_space(32.0);
                            
                            ui.horizontal(|ui| {
                                ui.add_space(ui.available_width() / 2.0 - 95.0);
                                
                                let ok_btn = ui.add_sized([80.0, 32.0], egui::Button::new("OK"));
                                if ok_btn.clicked() {
                                    clear_progress = true;
                                }
                                
                                ui.add_space(16.0);
                                
                                let open_folder_btn = ui.add_sized([100.0, 32.0], egui::Button::new("Open Folder"));
                                if open_folder_btn.clicked() {
                                    #[cfg(target_os = "windows")]
                                    {
                                        let _ = std::process::Command::new("explorer")
                                            .arg(&app.current_path)
                                            .spawn();
                                    }
                                    #[cfg(not(target_os = "windows"))]
                                    {
                                        let _ = std::process::Command::new("xdg-open")
                                            .arg(&app.current_path)
                                            .spawn();
                                    }
                                    clear_progress = true;
                                }
                            });
                        });
                    }
                    ProgressState::Error(err) => {
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new("❌").size(48.0).color(Color32::from_rgb(200, 50, 50)));
                            ui.add_space(8.0);
                            ui.label(RichText::new("Error").size(24.0).strong().color(Color32::WHITE));
                            ui.add_space(16.0);
                            ui.label(RichText::new(err).color(Color32::from_rgb(200, 100, 100)));
                            ui.add_space(32.0);
                            if ui.add_sized([80.0, 32.0], egui::Button::new("OK")).clicked() {
                                clear_progress = true;
                            }
                        });
                    }
                    _ => {}
                }
            }
        });

    if clear_progress {
        app.active_progress = None;
        app.refresh_dir();
    }
}
