pub mod album;
pub mod album_image;
pub mod auth;
pub mod cli;
pub mod store;
pub mod youtil;
use memory_stats::{MemoryStats, memory_stats};

use axum::{
    Router,
    body::{Body, Bytes},
    extract::{Path, Query, State},
    http::{HeaderValue, StatusCode, header},
    middleware,
    response::{Html, IntoResponse, Json, Redirect, Response},
    routing::{get, post},
    serve::Listener,
};

use image::ImageFormat;
use rust_embed::Embed;
// use std::borrow::Cow;

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::time::Instant;
use time::Duration as TDuration;
use tokio_js_set_interval::set_timeout_async;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::{Stream, StreamExt};
use tokio_util::io::ReaderStream;
use tower::ServiceBuilder;
use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer};
use webbrowser;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Stats {
    albums_count: usize,
    total_images: usize,
    cache_size_bytes: u64,
    server_uptime_seconds: u64,
    scaled_images_count: u64,
    downloaded_zips_count: u64,
    physical_mem: usize,
    virtual_mem: usize,
}

#[derive(Clone)]
pub struct AppState {
    pub base_path: String,
    pub prefix: String,
    pub single_album: String,
    pub filtered_extensions: Vec<String>,
    pub store: store::Store,
    pub anon: bool,
    pub browser_mode: bool,
    pub admin_secret: String,
    pub start_time: Instant,
    pub scaled_images: Arc<AtomicU64>,
    pub downloaded_zips: Arc<AtomicU64>,
}

#[tokio::main]
async fn main() {
    let (args, base, single_album, anon, browser_mode) = cli::get_cli_args_and_setup();

    let bind_host;
    let hostport;
    let http_prefix = format!("{}/", args.prefix.trim_end_matches('/'));
    let open_browser;
    let store = store::Store::new(&args.store);

    let app_state = AppState {
        base_path: base.clone(),
        single_album: single_album,
        filtered_extensions: args.extensions.split(',').map(|s| s.to_string()).collect(),
        store: store.clone(),
        prefix: http_prefix.clone(),
        anon: anon,
        browser_mode: browser_mode,
        admin_secret: auth::get_or_create_admin_secret(store),
        start_time: Instant::now(),
        scaled_images: Arc::new(AtomicU64::new(0)),
        downloaded_zips: Arc::new(AtomicU64::new(0)),
    };

    // let base = _base.unwrap_or(env::current_dir()?.to_string_lossy().to_string());
    // Create a shared state for our application. We use an Arc so that we clone the pointer to the state and
    // not the state itself. The AtomicU16 is a thread-safe integer that we use to keep track of the number of visits.

    match args.command {
        cli::Commands::Scan {} => {
            print!("Scanning for albums\n");
            album::build_alben(
                &app_state.base_path,
                &app_state.single_album,
                &app_state.filtered_extensions,
                &app_state.store,
            );
            return;
        }
        // cli::Commands::Stats { .. } => todo!(),
        cli::Commands::Stats { host, port } => {
            print!("Fetching stats from server at {}:{}\n", host, port);
            match fetch_stats(host, port).await {
                Ok(stats) => {
                    println!("{:#?}", stats);
                }
                Err(e) => println!("Error: {}", e),
            }
            // std::process::exit(0);
            return;
        }
        cli::Commands::Serve {
            host, port, open, ..
        } => {
            print!("Serving albums\n");
            hostport = format!("{}:{}", host, port).to_string();
            bind_host = host;
            album::build_alben(
                &app_state.base_path,
                &app_state.single_album,
                &app_state.filtered_extensions,
                &app_state.store,
            );
            open_browser = open;
        }
        cli::Commands::Browse { host, port } => {
            print!("Serving albums\n");
            hostport = format!("{}:{}", host, port).to_string();
            bind_host = host;
            album::build_alben(
                &app_state.base_path,
                &app_state.single_album,
                &app_state.filtered_extensions,
                &app_state.store,
            );
            open_browser = true;
        }
    }

    if &app_state.prefix != "/" {
        println!("Prefix: {}", &app_state.prefix);
    }

    let state = Arc::new(app_state);

    // let base_path = StdPath::new(&app_state.base_path);
    // print!("Base path: {:#?}\n", base_path.file_name().unwrap());

    // let serve_dir = ServeDir::new("public");
    // let serve_assets = ServeEmbed::<Assets>::new();

    // setup our application with "hello world" route at "/
    // let mut app = Router::new(); Router<Arc<AppState>>
    /*let mw = ServiceBuilder::new().layer(middleware::from_fn(
            // app_state.clone(),
            auth::check_auth_middleware,
        ));
    */

    println!("anon? {}", anon);

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_name("karton")
        .with_expiry(Expiry::OnInactivity(TDuration::seconds(60 * 1)));

    let album = Router::new()
        .route("/zip", get(download_zip))
        .route("/i/{size}/{img}", get(resize_image2))
        .route("/", get(show_album))
        .route("/", post(upload_image));
    let album = match anon {
        false => album
            .route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth::check_auth_middleware,
                // || auth.check_auth_middleware,
            ))
            .route_layer(session_layer),
        true => album,
    };

    let router = Router::new()
        // .route("/imagesize/{album}/{size}/{img}", get(resize_image)) // Placeholder route
        //.route("/{album}/zip", get(download_zip))
        //.route("/{album}/i/{size}/{img}", get(resize_image2)) // big size route
        //.route("/{album}", get(show_album))
        .nest("/a/{album}", album)
        .route("/_assets/{*file}", get(static_handler))
        // .nest_service("/_0assets", serve_dir.clone())
        // .nest_service("/_assets", serve_assets.clone())
        .route("/stats", get(stats_handler))
        .route("/", get(if_single_album_redirect))
        .with_state(state)
        .fallback_service(get(not_found));

    // let prefixed_router = Router::new().nest(&http_prefix, app);

    // cfg.prefix.unwrap_or("/")
    let router = match String::from(http_prefix.clone()).as_str() {
        "/" | "" => router,
        http_prefix => Router::new().nest(&http_prefix, router),
    };

    // start the server
    let listener = tokio::net::TcpListener::bind(hostport.clone()).await;
    let listener = match listener {
        Ok(l) => l,
        Err(msg) => {
            println!("unable to bind. trying different port ({})", msg);
            let hostport = format!("{}:0", bind_host);
            tokio::net::TcpListener::bind(hostport.clone())
                .await
                .unwrap()
        }
    };

    println!(
        "Listening on http://{:?}{}",
        listener.local_addr().ok().unwrap(),
        http_prefix
    );

    if open_browser {
        // let _ = after_start().await;
        let future = after_start(hostport.clone());
        set_timeout_async!(future, 600);
    }
    axum::serve(listener, router).await.unwrap();
}

async fn after_start(hostport: String) {
    println!("Opening Webbrowser");
    webbrowser::open(&format!("http://{}/", hostport)).unwrap();
}

async fn fetch_stats(host: String, port: u16) -> Result<Stats, Box<dyn std::error::Error>> {
    let resp = reqwest::get(format!("http://{}:{}/stats", host, port)).await?;
    if resp.status().is_success() {
        let stats: Stats = resp.json().await?;
        Ok(stats)
    } else {
        Err(format!("Server returned status: {}", resp.status()).into())
    }
}

async fn if_single_album_redirect(
    State(app_state): State<Arc<AppState>>,
) -> impl axum::response::IntoResponse {
    if app_state.single_album != "" {
        Redirect::permanent(&format!("{}a/{}", app_state.prefix, app_state.single_album))
            .into_response()
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

    let src = album::album_path(&app_state.base_path, &album);
    let cache = app_state.store.image_exists_in_cache(&src, &img, sz);

    // let mut buffer = BufWriter::new(Cursor::new(Vec::new()));

    let file_name = match cache {
        store::ImageFile::Found { path } => path,
        store::ImageFile::NotFound { path } => {
            let resized_img =
                album_image::resize_image(&app_state.base_path, &album, &img, sz).unwrap();
            resized_img
                .save_with_format(&path, ImageFormat::Jpeg)
                .unwrap();
            app_state
                .scaled_images
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            path
        }
    };

    let file = match tokio::fs::File::open(&file_name).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("resize failed {}", err))),
    };

    // convert the `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(file);
    // convert the `Stream` into an `axum::body::HttpBody`
    Ok((
        [(header::CONTENT_TYPE, "image/jpg")],
        Body::from_stream(stream),
    ))
}

async fn download_zip(
    State(app_state): State<Arc<AppState>>,
    Path(album): Path<String>,
) -> impl axum::response::IntoResponse {
    // let dispo = format!("attachment; filename=\"{}.zip\"", album);
    if let Some(zip_data) = album::zip(&app_state.base_path, &album, &app_state.filtered_extensions)
    {
        app_state
            .downloaded_zips
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let mut headers = axum::http::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/zip"),
        );
        headers.insert(
            header::CONTENT_DISPOSITION,
            HeaderValue::from_str(&format!("attachment; filename=\"{}.zip\"", album)).unwrap(),
        );
        return (headers, zip_data).into_response();
    }
    return "Album not found".into_response();
}

async fn show_album(
    State(app_state): State<Arc<AppState>>,
    Path(album): Path<String>,
) -> impl axum::response::IntoResponse {
    // Json(album::load(&app_state.base_path, &album))
    let album_data = album::load(&app_state.base_path, &album, &app_state.store);
    match album_data {
        Some(album) => {
            let html = album::render_index(&album, &app_state.prefix);
            ([(header::CONTENT_TYPE, "text/html")], html)
        }
        None => (
            [(header::CONTENT_TYPE, "text/html")],
            "Album not found".to_string(),
        ),
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

async fn stats_handler(State(app_state): State<Arc<AppState>>) -> Json<Stats> {
    let albums = youtil::list_dirs(&app_state.base_path);
    let albums_count = albums.len();
    let mut total_images = 0;
    for album_name in &albums {
        if let Some(album) = album::load(&app_state.base_path, album_name, &app_state.store) {
            total_images += album.images.len();
        }
    }
    let cache_size_bytes = calculate_cache_size(&app_state.store.cache_path);
    let server_uptime_seconds = app_state.start_time.elapsed().as_secs();
    let scaled_images_count = app_state
        .scaled_images
        .load(std::sync::atomic::Ordering::Relaxed);
    let downloaded_zips_count = app_state
        .downloaded_zips
        .load(std::sync::atomic::Ordering::Relaxed);
    let mem = match memory_stats() {
        Some(usage) => usage,
        None => MemoryStats {
            physical_mem: 0,
            virtual_mem: 0,
        },
    };
    Json(Stats {
        albums_count,
        total_images,
        cache_size_bytes,
        server_uptime_seconds,
        scaled_images_count,
        downloaded_zips_count,
        physical_mem: mem.physical_mem,
        virtual_mem: mem.virtual_mem,
    })
}

async fn upload_image(
    State(app_state): State<Arc<AppState>>,
    Path(album): Path<String>,
    Query(params): Query<std::collections::HashMap<String, String>>,
    body: Bytes,
) -> impl IntoResponse {
    let filename = params
        .get("filename")
        .unwrap_or(&"uploaded.jpg".to_string())
        .clone();
    let album_path = PathBuf::from(album::album_path(&app_state.base_path, &album));
    if !album_path.exists() {
        return (StatusCode::NOT_FOUND, "Album not found").into_response();
    }
    let file_path = album_path.join(filename);
    match tokio::fs::write(&file_path, body).await {
        Ok(_) => {
            // Optionally rebuild album index
            // album::build_alben(&app_state.base_path, &app_state.single_album, &app_state.filtered_extensions, &app_state.store);
            (StatusCode::OK, "Image uploaded successfully").into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to save image: {}", e),
        )
            .into_response(),
    }
}

fn calculate_cache_size(cache_path: &std::path::Path) -> u64 {
    let mut size = 0;
    if let Ok(entries) = std::fs::read_dir(cache_path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    size += metadata.len();
                } else if metadata.is_dir() {
                    size += calculate_cache_size(&entry.path());
                }
            }
        }
    }
    size
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
