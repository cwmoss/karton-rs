use crate::AppState;
use crate::youtil;
use axum::extract::Path;
use axum::extract::State;
use axum::response::IntoResponse;
use std::path::PathBuf;
use std::sync::Arc;

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
    let index = youtil::list_images_dirs_files(&dir, None);
    format!("index {:?}", index)
    /*
    let album_data = album::load(&app_state.base_path, &album, &app_state.store);
    match album_data {
        Some(album) => {
            let html = album::render_index(&album, &app_state.prefix, true);
            ([(header::CONTENT_TYPE, "text/html")], html)
        }
        None => (
            [(header::CONTENT_TYPE, "text/html")],
            "Album not found".to_string(),
        ),
    }
    */
}

pub async fn browse_dir() -> impl IntoResponse {
    format!("hier ist die base")
}
