use std::sync::{Arc, Mutex};

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
    },
    Finished {
        success: bool,
        message: String,
    },
    Error(String),
}

#[derive(Clone)]
pub struct Progress {
    state: Arc<Mutex<ProgressState>>,
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
        }
    }

    pub fn get_state(&self) -> ProgressState {
        self.state.lock().unwrap().clone()
    }

    pub fn set_state(&self, new_state: ProgressState) {
        *self.state.lock().unwrap() = new_state;
    }
}
