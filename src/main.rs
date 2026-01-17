pub mod album;
pub mod album_image;
pub mod store;
pub mod youtil;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    http::header,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{delete, get},
};

use image::ImageFormat;
use rust_embed::Embed;
use serde::{Deserialize, Serialize};
// use std::borrow::Cow;
use std::env;
use std::io::{BufWriter, Cursor};
use std::path::Path as StdPath;
use std::path::PathBuf;
use std::sync::{Arc, atomic::AtomicU16, atomic::Ordering::Relaxed};
use tower_http::{
    services::{ServeDir, ServeFile},
    // trace::TraceLayer,
};

use crate::youtil::list_dirs;

use clap::{Args, Parser, Subcommand, ValueEnum};
use directories::BaseDirs;

/// Karton serves your photo albums over HTTP.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Base path to albums, if not set,
    /// uses current directory or KARTON_BASE env var
    #[arg(short, long, default_value_t=get_default_base_path(), verbatim_doc_comment)]
    base: String,

    /// Base path to store, if not set,
    /// uses home directory/.karton  or KARTON_STORE env var
    #[arg(long, default_value_t=get_default_store_path(), verbatim_doc_comment)]
    store: String,

    /// Comma-separated list of filtered
    /// extensions (e.g., "jpg,jpeg,png")
    #[arg(long, default_value_t = String::from("jpg,jpeg"), verbatim_doc_comment)]
    extensions: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand, Eq, PartialEq)]
enum Commands {
    /// Start the Karton web server
    Serve {
        /// Host to bind to
        #[arg(long, default_value_t = String::from("0.0.0.0"))]
        host: String,

        #[arg(long, default_value_t = 5000)]
        port: u16,
    },
    /// Scan for albums and build caches
    Scan {},
}

struct AppState {
    base_path: String,
    single_album: String,
    filtered_extensions: Vec<String>,
    store: store::Store,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let path = StdPath::new(&args.base);
    let mut base = path.canonicalize().unwrap().to_string_lossy().to_string();

    let single_album = check_if_base_contains_jpgs(&base);

    if single_album == "" {
        print!("* Multi-album mode: {}/*/\n", base);
    } else {
        base = path.parent().unwrap().to_string_lossy().to_string();
        print!("* Single-album mode: {}/{}\n", base, single_album);
    }

    print!("Using store path: {}\n", args.store);

    //if single_album != "" {
    //    base = format!("{}/../", base);
    //}

    let hostport;

    // let base = _base.unwrap_or(env::current_dir()?.to_string_lossy().to_string());
    // Create a shared state for our application. We use an Arc so that we clone the pointer to the state and
    // not the state itself. The AtomicU16 is a thread-safe integer that we use to keep track of the number of visits.
    let app_state = Arc::new(AppState {
        base_path: base.clone(),
        single_album: single_album,
        filtered_extensions: args.extensions.split(',').map(|s| s.to_string()).collect(),
        store: store::Store::new(&args.store),
    });

    match args.command {
        Commands::Scan {} => {
            print!("Scanning for albums\n");
            album::build_alben(
                &app_state.base_path,
                &app_state.single_album,
                &app_state.filtered_extensions,
                &app_state.store,
            );
            return;
        }
        Commands::Serve { host, port } => {
            print!("Serving albums\n");
            hostport = format!("{}:{}", host, port).to_string();
            album::build_alben(
                &app_state.base_path,
                &app_state.single_album,
                &app_state.filtered_extensions,
                &app_state.store,
            );
        }
    }

    // let base_path = StdPath::new(&app_state.base_path);
    // print!("Base path: {:#?}\n", base_path.file_name().unwrap());

    // let serve_dir = ServeDir::new("public");
    // let serve_assets = ServeEmbed::<Assets>::new();

    // setup our application with "hello world" route at "/
    // let mut app = Router::new(); Router<Arc<AppState>>

    let app = Router::new()
        .route("/", get(if_single_album_redirect))
        // .route("/imagesize/{album}/{size}/{img}", get(resize_image)) // Placeholder route
        .route("/{album}/zip", get(download_zip))
        .route("/{album}/{size}/{img}", get(resize_image2)) // big size route
        .route("/{album}", get(show_album))
        .route("/_assets/{*file}", get(static_handler))
        // .nest_service("/_0assets", serve_dir.clone())
        // .nest_service("/_assets", serve_assets.clone())
        .with_state(app_state)
        .fallback_service(get(not_found));
    /*
        if app_state.single_album != "" {
            app_state.base_path = format!("{}/../", app_state.base_path);
            app = app.route(
                "/",
                get(|| async { Redirect::permanent(&format!("/{}", app_state.single_album)) }),
            );
        }
    */
    // start the server on port 3000
    let listener = tokio::net::TcpListener::bind(hostport.clone())
        .await
        .unwrap();

    println!("Listening on http://{}", hostport);

    axum::serve(listener, app).await.unwrap();
}

async fn if_single_album_redirect(
    State(app_state): State<Arc<AppState>>,
) -> impl axum::response::IntoResponse {
    if app_state.single_album != "" {
        Redirect::permanent(&format!("/{}", app_state.single_album)).into_response()
    } else {
        Html("hello, my name is karton").into_response()
    }
}
async fn resize_image2(
    State(app_state): State<Arc<AppState>>,
    Path((album, size, img)): Path<(String, String, String)>,
) -> impl axum::response::IntoResponse {
    let sz = match size.as_str() {
        "big" => album_image::get_size(album_image::Sizes::Big),
        _ => album_image::get_size(album_image::Sizes::Small),
    };
    // format!("Resizing image: album={}, size={} x {}, img={}",album, sz.0, sz.1, img)

    let mut buffer = BufWriter::new(Cursor::new(Vec::new()));
    album_image::resize_image(&app_state.base_path, &album, &img, sz)
        .unwrap()
        .write_to(&mut buffer, ImageFormat::Png)
        .unwrap();

    let bytes: Vec<u8> = buffer.into_inner().unwrap().into_inner();

    (
        axum::response::AppendHeaders([(header::CONTENT_TYPE, "image/jpg")]),
        bytes,
    )
}

async fn download_zip(
    State(app_state): State<Arc<AppState>>,
    Path(album): Path<String>,
) -> impl axum::response::IntoResponse {
    // let dispo = format!("attachment; filename=\"{}.zip\"", album);
    match album::zip(&app_state.base_path, &album, &app_state.filtered_extensions) {
        Some(zip_data) => {
            (
                axum::response::AppendHeaders([
                    (header::CONTENT_TYPE, "application/zip"),
                    (
                        header::CONTENT_DISPOSITION,
                        "attachment; filename=\"album.zip\"", // &*<std::string::String as Into<T>>::into(dispo),
                                                              // Cow::Owned(dispo),
                    ),
                ]),
                zip_data,
            )
        }
        None => (
            axum::response::AppendHeaders([
                (header::CONTENT_TYPE, "text/html"),
                (header::CONTENT_DISPOSITION, "inline"),
            ]),
            "Album not found".as_bytes().to_vec(),
        ),
    }
}

async fn show_album(
    State(app_state): State<Arc<AppState>>,
    Path(album): Path<String>,
) -> impl axum::response::IntoResponse {
    // Json(album::load(&app_state.base_path, &album))
    let album_data = album::load(&app_state.base_path, &album, &app_state.store);
    match album_data {
        Some(album) => {
            let html = album::render_index(&album);
            (
                axum::response::AppendHeaders([(header::CONTENT_TYPE, "text/html")]),
                html,
            )
        }
        None => (
            axum::response::AppendHeaders([(header::CONTENT_TYPE, "text/html")]),
            "Album not found".to_string(),
        ),
    }
}

fn get_default_store_path() -> String {
    match env::var("KARTON_STORE") {
        Ok(env_base) => env_base,
        Err(_) => {
            let base_dirs = BaseDirs::new().unwrap();
            let home_dir = base_dirs.home_dir();
            let store_path = home_dir.join(".karton");
            store_path.to_string_lossy().to_string()
        }
    }
    // if let Ok(env_base) = env::var("KARTON_BASE")
}

fn get_default_base_path() -> String {
    match env::var("KARTON_BASE") {
        Ok(env_base) => env_base,
        Err(_) => env::current_dir().unwrap().to_string_lossy().to_string(),
    }
    // if let Ok(env_base) = env::var("KARTON_BASE")
}

// We use a wildcard matcher ("/dist/*file") to match against everything
// within our defined assets directory. This is the directory on our Asset
// struct below, where folder = "examples/public/".
async fn static_handler(Path(path): Path<String>) -> impl IntoResponse {
    StaticFile(path)
}

// Finally, we use a fallback route for anything that didn't match.
async fn not_found() -> Html<&'static str> {
    Html("<h1>404</h1><p>Not Found</p>")
}

#[derive(Embed)]
#[folder = "public/"]
struct Asset;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match Asset::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
        }
    }
}

fn check_if_base_contains_jpgs(base: &str) -> String {
    let pattern = format!("{}/", base);
    print!("Checking if base contains jpgs in: {}\n", pattern);
    let files: Vec<PathBuf> = std::fs::read_dir(&pattern)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()?.to_str()? == "jpg" || path.extension()?.to_str()? == "jpeg" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    if files.len() > 0 {
        print!("Base contains jpg files, using single album mode.\n");
        let base_path = StdPath::new(&base);
        // print!("Base path: {:#?}\n", base_path.file_name().unwrap());
        return base_path.file_name().unwrap().to_str().unwrap().to_string();
    } else {
        print!("Base does not contain jpg files, using multi-album mode.\n");
        return "".to_string();
    }
}
