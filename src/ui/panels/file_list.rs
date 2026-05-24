use egui::{Context, CentralPanel, RichText, ScrollArea, Color32};
use crate::ui::app::LoserArApp;
use egui_extras::{TableBuilder, Column};

pub fn render(app: &mut LoserArApp, ctx: &Context) {
    CentralPanel::default().show(ctx, |ui| {
        // Path breadcrumbs
        ui.horizontal(|ui| {
            let path_str = app.current_path.to_string_lossy().to_string();
            ui.label(RichText::new(path_str).strong().size(14.0));
        });
        ui.add_space(4.0);
        ui.separator();

        let mut navigate_to = None;

        TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::initial(300.0).resizable(true)) // Name
            .column(Column::initial(100.0).resizable(true)) // Size
            .column(Column::remainder()) // Date
            .header(24.0, |mut header| {
                header.col(|ui| { ui.heading("Name"); });
                header.col(|ui| { ui.heading("Size"); });
                header.col(|ui| { ui.heading("Modified"); });
            })
            .body(|mut body| {
                for entry in &app.entries {
                    let is_selected = app.selected_paths.contains(&entry.path);
                    
                    body.row(24.0, |mut row| {
                        row.set_selected(is_selected);
                        
                        let icon = if entry.is_dir { "📁" } else { "📄" };
                        
                        row.col(|ui| {
                            let response = ui.selectable_label(is_selected, format!("{} {}", icon, entry.name));
                            if response.clicked() {
                                if ui.input(|i| i.modifiers.ctrl) {
                                    if is_selected {
                                        app.selected_paths.retain(|p| p != &entry.path);
                                    } else {
                                        app.selected_paths.push(entry.path.clone());
                                    }
                                } else {
                                    app.selected_paths = vec![entry.path.clone()];
                                }
                            }
                            if response.double_clicked() {
                                if entry.is_dir {
                                    navigate_to = Some(entry.path.clone());
                                } else if entry.name.ends_with(".zip") {
                                     // Handle opening zip
                                     app.selected_paths = vec![entry.path.clone()];
                                     app.show_extract_dialog = true;
                                }
                            }
                        });
                        row.col(|ui| {
                            if !entry.is_dir {
                                let size_mb = entry.size as f64 / 1_048_576.0;
                                ui.label(format!("{:.2} MB", size_mb));
                            }
                        });
                        row.col(|ui| {
                            if let Some(dt) = entry.modified {
                                ui.label(dt.format("%Y-%m-%d %H:%M").to_string());
                            }
                        });
                    });
                }
            });

        if let Some(new_path) = navigate_to {
            app.current_path = new_path;
            app.refresh_dir();
        }
    });
}
