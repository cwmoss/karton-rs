pub mod album;
pub mod album_image;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    http::header,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{delete, get},
};
use env;
use image::ImageFormat;
use rust_embed::Embed;
use serde::{Deserialize, Serialize};
// use std::borrow::Cow;
use std::io::{BufWriter, Cursor};
use std::path::Path as StdPath;
use std::path::PathBuf;
use std::sync::{Arc, atomic::AtomicU16, atomic::Ordering::Relaxed};

use tower_http::{
    services::{ServeDir, ServeFile},
    // trace::TraceLayer,
};

#[derive(Serialize, Deserialize)]
struct Greeting {
    greeting: String,
    visitor: String,
    visits: u16,
}

struct AppState {
    number_of_visits: AtomicU16,
    base_path: String,
    single_album: String,
}

impl Greeting {
    fn new(greeting: &str, visitor: String, visits: u16) -> Self {
        Greeting {
            greeting: greeting.to_string(),
            visitor,
            visits,
        }
    }
}

#[tokio::main]
async fn main() {
    let (mut base,) = get_args();

    let single_album = check_if_base_contains_jpgs(&base);

    print!("* Single album mode: '{}'\n", single_album);

    if single_album != "" {
        base = format!("{}/../", base);
    }

    // let base = _base.unwrap_or(env::current_dir()?.to_string_lossy().to_string());
    // Create a shared state for our application. We use an Arc so that we clone the pointer to the state and
    // not the state itself. The AtomicU16 is a thread-safe integer that we use to keep track of the number of visits.
    let app_state = Arc::new(AppState {
        number_of_visits: AtomicU16::new(1),
        base_path: base.clone(),
        single_album: single_album,
    });

    // let base_path = StdPath::new(&app_state.base_path);
    // print!("Base path: {:#?}\n", base_path.file_name().unwrap());
    // build_alben(&app_state.base_path);

    // let serve_dir = ServeDir::new("public");
    // let serve_assets = ServeEmbed::<Assets>::new();

    // setup our application with "hello world" route at "/
    // let mut app = Router::new(); Router<Arc<AppState>>

    let app = Router::new()
        .route("/", get(if_single_album_redirect))
        .route("/hello/{visitor}", get(greet_visitor))
        .route("/bye", delete(say_goodbye))
        .route("/imagesize/{album}/{size}/{img}", get(resize_image)) // Placeholder route
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
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
/*

$router->get('/imagesize/([-\w]+)/(\w+)/([-\w]+\.jpg)', function ($name, $size, $img) use ($gallery) {
    check_login($name);
    dbg("+++ resize", $name, $size, $img);
    $gallery->load($name);
    $gallery->image_resize($name, $img, $size);
});

*/
/// Extract the `visitor` path parameter and use it to greet the visitor.
/// We also use the `State` extractor to access the shared `AppState` and increment the number of visits.
/// We use `Json` to automatically serialize the `Greeting` struct to JSON.
async fn greet_visitor(
    State(app_state): State<Arc<AppState>>,
    Path(visitor): Path<String>,
) -> Json<Greeting> {
    let visits = app_state.number_of_visits.fetch_add(1, Relaxed);
    Json(Greeting::new("Hello", visitor, visits))
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
    let dispo = format!("attachment; filename=\"{}.zip\"", album);
    match album::zip(&app_state.base_path, &album) {
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

async fn resize_image(
    State(app_state): State<Arc<AppState>>,
    Path((album, size, img)): Path<(String, String, String)>,
) -> impl axum::response::IntoResponse {
    // album::resize_image(&album, &img, &size)
    let mut buffer = BufWriter::new(Cursor::new(Vec::new()));
    album::resize_image(&app_state.base_path, &album, &img, &size)
        .write_to(&mut buffer, ImageFormat::Png)
        .unwrap();

    let bytes: Vec<u8> = buffer.into_inner().unwrap().into_inner();

    (
        axum::response::AppendHeaders([(header::CONTENT_TYPE, "image/jpg")]),
        bytes,
    )
}

async fn show_album(
    State(app_state): State<Arc<AppState>>,
    Path(album): Path<String>,
) -> impl axum::response::IntoResponse {
    // Json(album::load(&app_state.base_path, &album))
    let album_data = album::load(&app_state.base_path, &album);
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
/// Say goodbye to the visitor.
async fn say_goodbye() -> String {
    "Goodbye".to_string()
}

fn get_args() -> (String,) {
    let args = std::env::args().collect::<Vec<String>>();
    /*if args.len() != 7 {
        eprintln!("Usage: {} INFILE OUTFILE X Y WIDTH HEIGHT", args[0]);
        std::process::exit(1);
    }*/
    (
        // args[1].to_owned()
        args.get(1)
            .unwrap_or(&env::current_dir().unwrap().to_string_lossy().to_string())
            .to_owned(),
        // args[6].parse().unwrap(),
    )
}

fn build_alben(base: &str) {
    let pattern = format!("{}/", base);
    print!("Building albums in pattern: {}\n", pattern);
    let albums: Vec<String> = std::fs::read_dir(&pattern)
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
        .collect();

    for album in albums {
        print!("Found album: {}\n", album);
        album::build_if_needed(base, &album);
    }
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
    let mut files: Vec<PathBuf> = std::fs::read_dir(&pattern)
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
