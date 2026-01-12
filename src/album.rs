use image::DynamicImage;
use image::GenericImageView;
use image::ImageFormat;
use image::ImageReader;
use std::fs;
use std::path::{Path, PathBuf};

pub fn list_files(base: &str, name: &str) -> Vec<FileInfo> {
    let pattern = format!("{}/var/{}/", base, name);
    print!("Listing files in pattern: {}\n", pattern);
    let mut files: Vec<PathBuf> = fs::read_dir(&pattern)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()?.to_str()? == "jpg" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    files.sort();

    files.into_iter().filter_map(|f| file_info(&f)).collect()
}

#[derive(Debug)]
pub struct FileInfo {
    fname: String,
    name: String,
    w: u32,
    h: u32,
    r#type: String,
    mime: String,
}

fn file_info(file: &Path) -> Option<FileInfo> {
    let img = ImageReader::open(file).ok()?.with_guessed_format().ok()?;
    let format = img.format()?;
    let img = img.decode().ok()?;
    let (w, h) = img.dimensions();

    let mime = match format {
        ImageFormat::Jpeg => "image/jpeg",
        _ => "application/octet-stream",
    };

    Some(FileInfo {
        fname: fs::canonicalize(file).ok()?.to_string_lossy().to_string(),
        name: file.file_name()?.to_string_lossy().to_string(),
        w,
        h,
        r#type: "jpg".to_string(),
        mime: mime.to_string(),
    })
}

pub fn resize_image(base: &str, album: &str, img: &str, size: &str) -> DynamicImage {
    // Placeholder implementation
    println!(
        "Resizing image '{}' in album '{}' to size '{}'",
        img, album, size
    );
    let file = format!("{}/var/{}/{}", base, album, img);
    let img = image::open(&file).unwrap();
    // let img = ImageReader::open(file).ok()?.with_guessed_format().ok()?;

    let resized_img = img.resize_to_fill(400, 150, image::imageops::FilterType::Lanczos3);
    return resized_img;
}
