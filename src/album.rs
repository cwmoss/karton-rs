use image::DynamicImage;
use image::GenericImageView;
use image::ImageFormat;
use image::ImageReader;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, serde::Serialize)]
pub struct Album {
    name: String,
    images: Vec<FileInfo>,
}

pub fn list_files(base: &str, name: &str) -> Vec<FileInfo> {
    let pattern = format!("{}/{}/", base, name);
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

pub fn build_if_needed(base: &str, name: &str) -> Album {
    let path = format!("{}/{}/.karton/", base, name);
    let index = format!("{}index.json", path);
    let cache = format!("{}cache/", path);

    if Path::new(&path).is_dir() {
        print!("karton exists, skipping build check.\n");
    } else {
        print!("karton missing, building album: {}\n", name);
        fs::create_dir_all(&path).unwrap();
        // Placeholder: actual build logic would go here
    }

    if !Path::new(&cache).is_dir() {
        fs::create_dir_all(&cache).unwrap()
    }

    let album = Album {
        name: name.to_string(),
        images: list_files(base, name),
    };
    let j = serde_json::to_string(&album).unwrap();
    print!("Album built: {:?}\n", j);
    fs::write(index, j).unwrap();
    album
}

#[derive(Debug, serde::Serialize)]
pub struct FileInfo {
    fname: String,
    name: String,
    w: u32,
    h: u32,
    r#type: String,
    mime: String,
}

fn file_info(file: &Path) -> Option<FileInfo> {
    /*
        return Some(FileInfo {
            fname: fs::canonicalize(file).ok()?.to_string_lossy().to_string(),
            name: file.file_name()?.to_string_lossy().to_string(),
            w: 800,
            h: 600,
            r#type: "jpg".to_string(),
            mime: "image/jpeg".to_string(),
        });
    */
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
    let file = format!("{}/var/{}/{}", base, album, img);

    println!(
        "Resizing image '{}' in album '{}' to size '{}' => {}",
        img, album, size, file
    );

    let img = image::open(&file).unwrap();
    // let img = ImageReader::open(file).ok()?.with_guessed_format().ok()?;

    let resized_img = img.resize_to_fill(400, 150, image::imageops::FilterType::Lanczos3);
    return resized_img;
}
