use std::fs;
use std::path::PathBuf;

pub fn list_files(base: &str, name: &str, filtered_extensions: &Vec<String>) -> Vec<PathBuf> {
    let pattern = format!("{}/{}/", base, name);
    // print!("Listing files in pattern: {}\n", pattern);
    fs::read_dir(&pattern)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let ext = path.extension()?.to_str()?.to_lowercase();
            if filtered_extensions.contains(&ext) {
                Some(path)
            } else {
                None
            }
        })
        .collect()
}

pub fn list_dirs(base: &str) -> Vec<String> {
    let dir = format!("{}", base);
    // print!("Listing directories in base: {}\n", dir);
    fs::read_dir(&dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_dir() {
                Some(path.file_name()?.to_str()?.to_string())
            } else {
                None
            }
        })
        .collect()
}

pub fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    format!("{}d {}h {}m", days, hours, minutes)
}

pub fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    format!("{:.1} {}", size, UNITS[unit_index])
}

/*
std::fs::read_dir(&pattern)
            .unwrap()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_dir() {
                    Some(path.file_name()?.to_string_lossy().to_string())
                } else {
                    None
                }
            })
            .collect() */
