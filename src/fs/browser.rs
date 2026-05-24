use anyhow::Result;
use chrono::{DateTime, Local, TimeZone};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: u64,
    pub modified: Option<DateTime<Local>>,
}

pub fn browse_directory(path: &Path) -> Result<Vec<FileEntry>> {
    let mut entries = Vec::new();

    if let Some(parent) = path.parent() {
        if path.parent().is_some() {
            entries.push(FileEntry {
                name: "..".to_string(),
                path: parent.to_path_buf(),
                is_dir: true,
                size: 0,
                modified: None,
            });
        }
    }

    for entry_result in fs::read_dir(path)? {
        let entry = entry_result?;
        let metadata = entry.metadata()?;
        let path_buf = entry.path();
        
        let modified = metadata.modified().ok().map(|sys_time| {
            let dt: DateTime<Local> = sys_time.into();
            dt
        });

        entries.push(FileEntry {
            name: entry.file_name().to_string_lossy().to_string(),
            path: path_buf,
            is_dir: metadata.is_dir(),
            size: if metadata.is_dir() { 0 } else { metadata.len() },
            modified,
        });
    }

    // Sort: directories first, then alphabetical
    entries.sort_by(|a, b| {
        if a.name == ".." {
            return std::cmp::Ordering::Less;
        }
        if b.name == ".." {
            return std::cmp::Ordering::Greater;
        }
        match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });

    Ok(entries)
}

#[cfg(windows)]
pub fn list_drives() -> Vec<PathBuf> {
    let mut drives = Vec::new();
    let mut mask = unsafe { winapi::um::fileapi::GetLogicalDrives() };
    let mut letter = b'A';
    
    while mask > 0 {
        if mask & 1 != 0 {
            drives.push(PathBuf::from(format!("{}:\\", letter as char)));
        }
        mask >>= 1;
        letter += 1;
    }
    drives
}

#[cfg(not(windows))]
pub fn list_drives() -> Vec<PathBuf> {
    vec![PathBuf::from("/")]
}
