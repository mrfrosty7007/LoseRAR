use chrono::{DateTime, Local};

#[derive(Clone, Debug)]
pub struct ArchiveEntry {
    pub name: String,
    pub size: u64,
    pub compressed_size: u64,
    pub is_dir: bool,
    pub modified: Option<DateTime<Local>>,
}
