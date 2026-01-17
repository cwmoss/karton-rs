use std::fs;
use std::path::PathBuf;

pub fn list_files(base: &str, name: &str, filtered_extensions: &Vec<String>) -> Vec<PathBuf> {
    let pattern = format!("{}/{}/", base, name);
    print!("Listing files in pattern: {}\n", pattern);
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
