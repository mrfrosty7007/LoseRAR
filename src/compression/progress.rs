use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
#[derive(Clone, Debug)]
pub enum ProgressState {
    Idle,

    Scanning,

    Working {
        current_file: String,
        files_processed: usize,
        total_files: usize,
        bytes_processed: u64,
        total_bytes: u64,
        start_time: std::time::Instant,
    },

    Finished {
        success: bool,

        archive_name: String,
        file_count: usize,

        original_size: u64,
        compressed_size: u64,

        duration_secs: f64,

        message: String,
    },

    Error(String),
}

#[derive(Clone)]
pub struct Progress {
    state: Arc<Mutex<ProgressState>>,
    cancelled: Arc<AtomicBool>,
}

impl Default for Progress {
    fn default() -> Self {
        Self::new()
    }
}

impl Progress {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(ProgressState::Idle)),
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn get_state(&self) -> ProgressState {
        self.state.lock().unwrap().clone()
    }

    pub fn set_state(&self, new_state: ProgressState) {
        *self.state.lock().unwrap() = new_state;
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }
}
