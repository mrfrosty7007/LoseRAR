use crate::compression::ProgressState;
use crate::compression::Progress;
use anyhow::{Context, Result};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn extract_archive(
    archive_path: PathBuf,
    dest_dir: PathBuf,
    progress: Progress,
) -> Result<()> {
    progress.set_state(ProgressState::Scanning);

    let file = fs::File::open(&archive_path).context("Failed to open archive file")?;
    let mut archive = zip::ZipArchive::new(file)?;
    
    let total_files = archive.len();
    if total_files == 0 {
        progress.set_state(ProgressState::Error("Archive is empty.".into()));
        return Ok(());
    }

    // Rough estimate, some archives don't provide uncompressed size easily
    let mut total_bytes: u64 = 0;
    for i in 0..archive.len() {
        if let Ok(file) = archive.by_index(i) {
            total_bytes += file.size();
        }
    }

    fs::create_dir_all(&dest_dir)?;

    let mut files_processed = 0;
    let mut bytes_processed = 0;
    let start_time = std::time::Instant::now();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => dest_dir.join(path),
            None => continue,
        };

        let file_name_str = file.name().to_string();

        progress.set_state(ProgressState::Working {
            current_file: file_name_str.clone(),
            files_processed,
            total_files,
            bytes_processed,
            total_bytes,
            start_time,
        });

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            
            // Chunked copying for progress reporting
            let mut buffer = [0; 65536];
            loop {
                if progress.is_cancelled() {
                    progress.set_state(ProgressState::Error("Extraction cancelled.".to_string()));
                    return Ok(());
                }

                let count = io::Read::read(&mut file, &mut buffer)?;
                if count == 0 {
                    break;
                }
                io::Write::write_all(&mut outfile, &buffer[..count])?;
                bytes_processed += count as u64;

                if bytes_processed % (4 * 1024 * 1024) < count as u64 {
                    progress.set_state(ProgressState::Working {
                        current_file: file_name_str.clone(),
                        files_processed,
                        total_files,
                        bytes_processed,
                        total_bytes,
                        start_time,
                    });
                }
            }
        }
        
        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap_or_default();
            }
        }

        files_processed += 1;
    }

progress.set_state(ProgressState::Finished {
    success: true,

    archive_name: archive_path
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string(),

    file_count: files_processed,

    original_size: 0,
    compressed_size: 0,

    duration_secs: start_time.elapsed().as_secs_f64(),

    message: format!(
        "Successfully extracted {} files.",
        files_processed
    ),
});

    Ok(())
}
