use crate::AppState;
use crate::view::render_browser;
use crate::youtil;
use axum::extract::Path;
use axum::extract::State;
use axum::response::IntoResponse;
// use serde;
use axum::response::Html;
use serde_json::json;
use std::path::PathBuf;
use std::sync::Arc;

pub struct DirPage {
    pub source: String,
    pub name: String,
    pub images: Vec<youtil::ImageInfo>,
    pub dirs: Vec<String>,
}
pub async fn browse_subdir(
    State(app_state): State<Arc<AppState>>,
    subpath: Option<Path<String>>,
) -> impl IntoResponse {
    let base = PathBuf::from(&app_state.base_path);
    let dir = match subpath {
        None => base,
        Some(subpath) => PathBuf::from(&app_state.base_path).join(&subpath.to_string()),
    };
    println!("hier ist das dir {:?}", &dir);
    let index = youtil::list_images_dirs_files(&dir, None).unwrap();
    println!("{}", format!("index {:?}", index));

    Html(render_browser(
        DirPage {
            source: "".to_string(),
            name: dir.file_name().unwrap().to_string_lossy().to_string(),
            images: index.0,
            dirs: index.2,
        },
        &app_state.prefix,
        false,
    ))
}
