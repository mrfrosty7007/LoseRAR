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
                self.selected_paths = paths;
                self.show_compress_dialog = true;
            }
        }
    }
}
