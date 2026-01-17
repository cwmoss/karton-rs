use super::youtil::list_files;
use askama::Template;
use image::DynamicImage;
use image::GenericImageView;
use image::ImageFormat;
use image::ImageReader;
use serde;
use std::fs;
use std::path::{Path, PathBuf};
use zip::write::SimpleFileOptions;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Album {
    name: String,
    images: Vec<FileInfo>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FileInfo {
    fname: String,
    name: String,
    w: u32,
    h: u32,
    r#type: String,
    mime: String,
}

pub fn load(base: &str, name: &str) -> Option<Album> {
    let path = format!("{}/{}/.karton/index.json", base, name);
    let data = fs::read_to_string(&path).ok()?;
    let album: Album = serde_json::from_str(&data).ok()?;
    Some(album)
}

pub fn list_files_with_info(
    base: &str,
    name: &str,
    filtered_extensions: &Vec<String>,
) -> Vec<FileInfo> {
    list_files(base, name, filtered_extensions)
        .into_iter()
        .filter_map(|f| file_info(&f))
        .collect()
}

pub fn build_if_needed(base: &str, name: &str, filtered_extensions: &Vec<String>) -> Album {
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
        images: list_files_with_info(base, name, filtered_extensions),
    };
    let j = serde_json::to_string(&album).unwrap();
    print!("Album built: {:?}\n", j);
    fs::write(index, j).unwrap();
    album
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

#[derive(Template)] // this will generate the code...
#[template(path = "index.html")]
// using the template in this path, relative
// to the `templates` dir in the crate root
struct IndexTemplate<'a> {
    // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
    // in your template
    album: &'a str,
    total: usize,
}

pub fn render_index(album: &Album) -> String {
    let album_json = serde_json::to_string(&album).unwrap();
    let template = IndexTemplate {
        name: album.name.as_str(),
        album: &album_json,
        total: album.images.len(),
    };
    template.render().unwrap()
}

pub fn zip(base: &str, album: &str, filtered_extensions: &Vec<String>) -> Option<Vec<u8>> {
    // let album_path = format!("{}/{}", base, album);
    let zip_path = format!("{}/{}/.karton/{}.zip", base, album, album);

    let file = fs::File::create(&zip_path).ok()?;
    let mut zip = zip::ZipWriter::new(file);

    // let options = zip::write::FileOptions::default();

    let files_to_compress: Vec<PathBuf> = list_files(base, album, filtered_extensions);
    // Iterate through the files and add them to the ZIP archive.
    for file_path in &files_to_compress {
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        print!("Adding file to zip: {}\n", file_name);

        let mut file = fs::File::open(file_path).ok()?;

        // Adding the file to the ZIP archive.
        zip.start_file(
            file_name,
            SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored),
        )
        .ok()?;

        let _ = std::io::copy(&mut file, &mut zip);
    }

    zip.finish().ok()?;
    print!("Closing ZIP file\n");

    let zip_data = fs::read(&zip_path).ok()?;
    Some(zip_data)
}
