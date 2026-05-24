use std::path::PathBuf;
use crate::fs::FileEntry;
use crate::compression::{Progress, ProgressState};

pub struct LoserArApp {
    pub current_path: PathBuf,
    pub entries: Vec<FileEntry>,
    pub selected_paths: Vec<PathBuf>,
    
    // Dialog state
    pub show_compress_dialog: bool,
    pub show_extract_dialog: bool,
    
    // Background task progress
    pub active_progress: Option<Progress>,
}

impl Default for LoserArApp {
    fn default() -> Self {
        let default_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("C:\\"));
        let mut app = Self {
            current_path: default_path.clone(),
            entries: Vec::new(),
            selected_paths: Vec::new(),
            show_compress_dialog: false,
            show_extract_dialog: false,
            active_progress: None,
        };
        app.refresh_dir();
        app
    }
}

impl LoserArApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    pub fn refresh_dir(&mut self) {
        if let Ok(entries) = crate::fs::browse_directory(&self.current_path) {
            self.entries = entries;
        }
        self.selected_paths.clear();
    }
    
    pub fn check_progress(&mut self) {
        if let Some(progress) = &self.active_progress {
            let state = progress.get_state();
            if matches!(state, ProgressState::Finished { .. } | ProgressState::Error(_)) {
                // Keep showing it for a moment or let the status bar handle dismissal
            }
        }
    }
}

impl eframe::App for LoserArApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.check_progress();

        crate::ui::panels::toolbar::render(self, ctx);
        crate::ui::panels::status_bar::render(self, ctx);
        crate::ui::panels::sidebar::render(self, ctx);
        crate::ui::panels::file_list::render(self, ctx);

        crate::ui::dialogs::compress::render(self, ctx);
        crate::ui::dialogs::extract::render(self, ctx);

        // Draw visual highlight when dragging over the window
        if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
            let screen_rect = ctx.screen_rect();
            egui::Area::new(egui::Id::new("drag_drop_overlay"))
                .fixed_pos(screen_rect.min)
                .order(egui::Order::Foreground)
                .interactable(false)
                .show(ctx, |ui| {
                    // Draw semi-transparent dark background
                    let painter = ui.painter();
                    painter.rect_filled(
                        screen_rect,
                        10.0, // Match window rounding
                        egui::Color32::from_rgba_unmultiplied(20, 20, 24, 210), // Sleek, modern dark glassmorphism
                    );

                    // Draw a dashed glowing accent border
                    let stroke_color = egui::Color32::from_rgb(88, 101, 242); // Theme's accent blurple
                    let stroke = egui::Stroke::new(2.5, stroke_color);
                    painter.rect_stroke(
                        screen_rect.shrink(24.0),
                        10.0,
                        stroke,
                    );

                    // Centered feedback content
                    ui.allocate_ui_at_rect(screen_rect.shrink(32.0), |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(screen_rect.height() / 2.0 - 100.0);

                            ui.label(egui::RichText::new("📥").size(64.0));
                            ui.add_space(16.0);

                            ui.label(
                                egui::RichText::new("Drop Files or Folders")
                                    .color(egui::Color32::WHITE)
                                    .size(24.0)
                                    .strong(),
                            );
                            ui.add_space(8.0);
                            ui.label(
                                egui::RichText::new("to automatically view and select them in LoseRAR")
                                    .color(egui::Color32::from_rgb(170, 170, 180))
                                    .size(14.0),
                            );
                        });
                    });
                });
        }

        // Handle drag and drop
        if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
            let dropped_files = ctx.input(|i| i.raw.dropped_files.clone());
            let mut paths = Vec::new();
            for file in dropped_files {
                if let Some(path) = file.path {
                    paths.push(path);
                }
            }
            if !paths.is_empty() {
                // Determine the parent folder of the first dropped path to navigate to it
                if let Some(parent) = paths[0].parent() {
                    self.current_path = parent.to_path_buf();
                    self.refresh_dir();
                } else {
                    // Fallback to the path itself if it has no parent (e.g. it is a root drive)
                    self.current_path = paths[0].clone();
                    self.refresh_dir();
                }
                self.selected_paths = paths;
            }
        }
    }
}
