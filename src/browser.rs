use crate::AppState;
use crate::album_image;
use crate::view::render_browser;
use crate::youtil;
use axum::body::{Body, Bytes};
use axum::extract::Path;
use axum::extract::State;
use axum::http::header;
use axum::response::IntoResponse;
// use serde;
use axum::response::Html;
use image::ImageFormat;
use serde_json::json;
use std::path::PathBuf;
use std::sync::Arc;

use std::io::{BufWriter, Cursor};

pub struct DirPage {
    pub source: String,
    pub name: String,
    pub path: String,
    pub images: Vec<youtil::ImageInfo>,
    pub dirs: Vec<String>,
}
pub async fn browse_subdir(
    State(app_state): State<Arc<AppState>>,
    subpath: Option<Path<String>>,
) -> impl IntoResponse {
    let base = PathBuf::from(&app_state.base_path);
    let dir = match subpath {
        None => &"".to_string(),
        Some(subpath) => &subpath.to_string(),
    };
    let path = PathBuf::from(&app_state.base_path).join(dir);
    println!("hier ist das dir {:?}", &path);
    let index = youtil::list_images_dirs_files(&path, None).unwrap();
    println!("{}", format!("index {:?}", index));

    Html(render_browser(
        DirPage {
            source: "".to_string(),
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            path: dir.to_string(),
            images: index.0,
            dirs: index.2,
        },
        &app_state.prefix,
        false,
    ))
}

// no cache, no album, img with subpath
pub async fn resize_image_browsing(
    State(app_state): State<Arc<AppState>>,
    Path((size, img)): Path<(String, String)>,
) -> impl axum::response::IntoResponse {
    let sz = match size.as_str() {
        "big" => album_image::get_size(album_image::Sizes::Big),
        _ => album_image::get_size(album_image::Sizes::Small),
    };
    // format!("Resizing image: album={}, size={} x {}, img={}",album, sz.0, sz.1, img)

    let src = PathBuf::from(&app_state.base_path).join(img);

    let resized_img = album_image::resize_image_path(&src, sz).unwrap();
    let mut buffer = BufWriter::new(Cursor::new(Vec::new()));
    resized_img
        .write_to(&mut buffer, ImageFormat::Jpeg)
        .unwrap();

    let bytes: Vec<u8> = buffer.into_inner().unwrap().into_inner();
    app_state
        .scaled_images
        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    ([(header::CONTENT_TYPE, "image/jpg")], bytes)
}
