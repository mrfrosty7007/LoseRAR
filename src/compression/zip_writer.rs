use super::progress::{Progress, ProgressState};
use anyhow::{Context, Result};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use zip::write::FileOptions;
use zip::CompressionMethod;

pub fn compress_paths(
    source_paths: Vec<PathBuf>,
    dest_archive: PathBuf,
    level: i32,
    progress: Progress,
) -> Result<()> {
    progress.set_state(ProgressState::Scanning);

    // 1. Scan to find total files and size
    let mut files_to_compress = Vec::new();
    let mut total_bytes = 0;

    for source_path in &source_paths {
        if source_path.is_dir() {
            for entry in WalkDir::new(source_path) {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    let metadata = entry.metadata()?;
                    total_bytes += metadata.len();
                    files_to_compress.push((path.to_path_buf(), path.strip_prefix(source_path.parent().unwrap_or(source_path))?.to_path_buf()));
                } else if path.is_dir() {
                     files_to_compress.push((path.to_path_buf(), path.strip_prefix(source_path.parent().unwrap_or(source_path))?.to_path_buf()));
                }
            }
        } else if source_path.is_file() {
            let metadata = std::fs::metadata(source_path)?;
            total_bytes += metadata.len();
            files_to_compress.push((source_path.clone(), PathBuf::from(source_path.file_name().unwrap())));
        }
    }

    let total_files = files_to_compress.len();
    if total_files == 0 {
        progress.set_state(ProgressState::Error("No files found to compress.".into()));
        return Ok(());
    }

    // 2. Create archive
    let file = File::create(&dest_archive).context("Failed to create archive file")?;
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .compression_level(Some(level))
        .unix_permissions(0o755);

    let mut files_processed = 0;
    let start_time =
    std::time::Instant::now();
    let mut bytes_processed = 0;

    for (src_path, archive_name) in files_to_compress {
        if progress.is_cancelled() {
            progress.set_state(ProgressState::Error("Compression cancelled.".to_string()));
            return Ok(());
        }

        let name_str = archive_name.to_string_lossy().replace("\\", "/");
        
        progress.set_state(ProgressState::Working {
            current_file: name_str.clone(),
            files_processed,
            total_files,
            bytes_processed,
            total_bytes,
            start_time,
        });

        if src_path.is_file() {
            zip.start_file(&name_str, options)?;
            let mut f = File::open(&src_path)?;
            let mut buffer = [0; 65536]; // 64KB chunks
            loop {
                if progress.is_cancelled() {
                    progress.set_state(ProgressState::Error("Compression cancelled.".to_string()));
                    return Ok(());
                }

                let count = f.read(&mut buffer)?;
                if count == 0 {
                    break;
                }
                zip.write_all(&buffer[..count])?;
                bytes_processed += count as u64;
                
// Update progress every 4 MB
if bytes_processed % (4 * 1024 * 1024) < count as u64 {

    progress.set_state(ProgressState::Working {
        current_file: name_str.clone(),
        files_processed,
        total_files,
        bytes_processed,
        total_bytes,
        start_time,
    });
}
            }
        } else if src_path.is_dir() && !name_str.is_empty() {
            zip.add_directory(&name_str, options)?;
        }
        
        files_processed += 1;
    }

zip.finish()?;

// Calculate statistics
let duration_secs =
    start_time.elapsed().as_secs_f64();

let compressed_size =
    std::fs::metadata(&dest_archive)?
        .len();
progress.set_state(ProgressState::Finished {
    success: true,

    archive_name: dest_archive
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string(),

    file_count: files_processed,

    original_size: total_bytes,
    compressed_size,

    duration_secs,

    message: format!(
        "Successfully created archive with {} files.",
        files_processed
    ),
});

    Ok(())
}
