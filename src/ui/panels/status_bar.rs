use egui::{Context, TopBottomPanel, ProgressBar};
use crate::ui::app::LoserArApp;
use crate::compression::ProgressState;

pub fn render(app: &mut LoserArApp, ctx: &Context) {
    TopBottomPanel::bottom("status_bar")
        .exact_height(75.0)
        .show(ctx, |ui| {
            ui.add_space(4.0); // Top padding

            let mut is_working = false;

            if let Some(progress) = &app.active_progress {
                let state = progress.get_state();

                match state {
                    ProgressState::Scanning => {
                        is_working = true;
                        ui.horizontal(|ui| {
                            ui.label(format!("{} items selected", app.selected_paths.len()));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.spinner();
                                ui.label("Scanning files...");
                            });
                        });
                    }
                    ProgressState::Working {
                        current_file,
                        files_processed: _,
                        total_files: _,
                        bytes_processed,
                        total_bytes,
                        start_time,
                    } => {
                        is_working = true;
                        
                        // Top row: Full width progress bar
                        let pct = if total_bytes > 0 {
                            bytes_processed as f32 / total_bytes as f32
                        } else {
                            0.0
                        };

                        let elapsed = start_time.elapsed().as_secs_f64();
                        
                        let speed_id = egui::Id::new("smoothed_speed");
                        let last_time_id = egui::Id::new("last_time");
                        let last_bytes_id = egui::Id::new("last_bytes");

                        let speed_mb_s = ctx.data_mut(|d| {
                            let now = elapsed;
                            let last_time: f64 = d.get_temp(last_time_id).unwrap_or(0.0);
                            let last_bytes: u64 = d.get_temp(last_bytes_id).unwrap_or(0);
                            let mut smoothed: f64 = d.get_temp(speed_id).unwrap_or(0.0);

                            let dt = now - last_time;
                            if dt > 0.5 { // Update instantaneous speed every 0.5s
                                let d_bytes = bytes_processed.saturating_sub(last_bytes);
                                let inst_speed = (d_bytes as f64 / 1_048_576.0) / dt;
                                
                                if smoothed == 0.0 {
                                    smoothed = inst_speed;
                                } else {
                                    // Smooth out fluctuations
                                    smoothed = smoothed * 0.7 + inst_speed * 0.3;
                                }
                                
                                d.insert_temp(last_time_id, now);
                                d.insert_temp(last_bytes_id, bytes_processed);
                                d.insert_temp(speed_id, smoothed);
                            }
                            
                            // Return global average if we don't have enough data yet
                            if smoothed == 0.0 && elapsed > 0.0 {
                                (bytes_processed as f64 / 1_048_576.0) / elapsed
                            } else {
                                smoothed
                            }
                        });

                        let remaining_bytes = total_bytes.saturating_sub(bytes_processed);
                        let speed_b_s = speed_mb_s * 1_048_576.0;
                        
                        let eta_str = if speed_b_s > 0.0 && remaining_bytes > 0 {
                            let eta_secs = remaining_bytes as f64 / speed_b_s;
                            if eta_secs < 1.0 {
                                "Finalizing...".to_string()
                            } else {
                                let mins = (eta_secs / 60.0).floor() as u64;
                                let secs = (eta_secs % 60.0).floor() as u64;
                                if mins > 0 {
                                    format!("{}m {}s remaining", mins, secs)
                                } else {
                                    format!("{}s remaining", secs)
                                }
                            }
                        } else if remaining_bytes == 0 || pct >= 1.0 {
                            "Finalizing...".to_string()
                        } else {
                            "Calculating...".to_string()
                        };

                        ui.add_sized(
                            [ui.available_width(), 16.0],
                            ProgressBar::new(pct).show_percentage(),
                        );

                        ui.add_space(6.0);

                        let short_filename = std::path::Path::new(&current_file)
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy();

                        // Second row: Label on left, Cancel on right
                        ui.horizontal(|ui| {
                            ui.label(format!(
                                "Processing: {} | {:.1}% • {:.1} MB/s • {}",
                                short_filename,
                                pct * 100.0,
                                speed_mb_s,
                                eta_str
                            ));
                            
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                let cancel_btn = egui::Button::new(
                                    egui::RichText::new("Cancel").color(egui::Color32::WHITE)
                                ).fill(egui::Color32::from_rgb(180, 40, 40));

                                if ui.add(cancel_btn).clicked() {
                                    progress.cancel();
                                }
                            });
                        });

                        ctx.request_repaint();
                    }
                    _ => {} // Idle, Finished, Error handled below or by modal
                }
            }

            if !is_working {
                ui.horizontal(|ui| {
                    ui.label(format!("{} items selected", app.selected_paths.len()));
                });
            }
        });
}