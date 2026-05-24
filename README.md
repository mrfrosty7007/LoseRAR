# LoserRAR

A modern, open-source alternative to WinRAR, built in Rust with `egui`.

## Features
- Modern dark theme GUI
- Create ZIP archives
- Extract ZIP archives
- File browser
- Multi-threaded operations (planned via Rayon)
- Progress tracking

## Building and Running

1. Ensure you have Rust installed.
2. Run the application:
   ```bash
   cargo run --release
   ```

## Architecture
See `docs/` or the code structure for more details. Uses `eframe`/`egui` for the frontend and `zip` for the archive operations.
