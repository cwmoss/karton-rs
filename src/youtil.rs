use std::fs;
use std::io::Error;
use std::path::PathBuf;

// use axum::Error;
// use std::vec::Vec

#[derive(Debug)]
pub struct ImageInfo {
    path: PathBuf,
    mime: String,
    w: usize,
    h: usize,
}
#[derive(Debug)]
pub struct FileInfo {
    path: PathBuf,
    mime: String,
}
pub fn list_images_dirs_files(
    base: &PathBuf,
    filtered_extensions: Option<&Vec<String>>,
) -> Result<(Vec<ImageInfo>, Vec<FileInfo>, Vec<PathBuf>), Error> {
    // et pattern = format!("{}/{}/", base, name);
    println!("Listing files in DIR: {:?}\n", base);
    Ok(fs::read_dir(&base)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.file_name()?.to_str()?.starts_with(".") {
                return None;
            }
            if let Some(filter) = filtered_extensions {
                let ext = path.extension()?.to_str()?.to_lowercase();
                if filter.contains(&ext) {
                    Some(path)
                } else {
                    None
                }
            } else {
                Some(path)
            }
        })
        .fold((Vec::new(), Vec::new(), Vec::new()), |mut acc, it| {
            if it.is_dir() {
                acc.2.push(it);
            } else {
                let (mime, ext, is_image) = get_mime_type_and_is_image(&it);
                if is_image {
                    let size = get_image_size_from_file(&it);
                    acc.0.push(ImageInfo {
                        path: it,
                        mime,
                        w: size.0,
                        h: size.1,
                    });
                } else {
                    acc.1.push(FileInfo { path: it, mime });
                }
            }
            acc
        }))
    // .collect()
}

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

// https://docs.rs/imagesize/0.14.0/imagesize/
// alt: https://github.com/xiaozhuai/imageinfo-rs
pub fn get_image_size(buf: &[u8]) -> (usize, usize) {
    match imagesize::blob_size(&buf) {
        Ok(size) => (size.width, size.height),
        Err(_) => (0, 0),
    }
}

pub fn get_image_size_from_file(path: &PathBuf) -> (usize, usize) {
    match imagesize::size(path) {
        Ok(size) => (size.width, size.height),
        Err(_) => (0, 0),
    }
}

pub fn get_mime_type_and_is_image(path: &PathBuf) -> (String, String, bool) {
    // let buf = [0xFF, 0xD8, 0xFF, 0xAA];
    if let Ok(Some(kind)) = infer::get_from_path(path) {
        (
            kind.mime_type().to_string(),
            kind.extension().to_string(),
            kind.mime_type().starts_with("image/"),
        )
    } else {
        ("application/octetstream".to_string(), "".to_string(), false)
    }
    // assert_eq!(kind.mime_type(), "image/jpeg");
    // assert_eq!(kind.extension(), "jpg");
}

pub fn get_mime_type(buf: &[u8]) -> (String, String) {
    // let buf = [0xFF, 0xD8, 0xFF, 0xAA];
    if let Some(kind) = infer::get(&buf) {
        (kind.mime_type().to_string(), kind.extension().to_string())
    } else {
        ("application/octetstream".to_string(), "".to_string())
    }
    // assert_eq!(kind.mime_type(), "image/jpeg");
    // assert_eq!(kind.extension(), "jpg");
}
