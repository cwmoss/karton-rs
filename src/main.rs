pub mod album;
pub mod album_image;
pub mod cli;
pub mod store;
pub mod youtil;

use axum::{
    Router,
    body::Body,
    extract::{Path, State},
    http::header,
    http::{HeaderValue, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
};

use image::ImageFormat;
use rust_embed::Embed;
// use std::borrow::Cow;

use std::sync::Arc;
use tokio_js_set_interval::set_timeout_async;
use tokio_util::io::ReaderStream;
use webbrowser;

struct AppState {
    base_path: String,
    prefix: String,
    single_album: String,
    filtered_extensions: Vec<String>,
    store: store::Store,
}

#[tokio::main]
async fn main() {
    let (args, base, single_album) = cli::get_cli_args_and_setup();

    let hostport;
    let http_prefix = format!("{}/", args.prefix.trim_end_matches('/'));
    let open_browser;

    // let base = _base.unwrap_or(env::current_dir()?.to_string_lossy().to_string());
    // Create a shared state for our application. We use an Arc so that we clone the pointer to the state and
    // not the state itself. The AtomicU16 is a thread-safe integer that we use to keep track of the number of visits.
    let app_state = Arc::new(AppState {
        base_path: base.clone(),
        single_album: single_album,
        filtered_extensions: args.extensions.split(',').map(|s| s.to_string()).collect(),
        store: store::Store::new(&args.store),
        prefix: http_prefix.clone(),
    });

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
        cli::Commands::Serve { host, port, open } => {
            print!("Serving albums\n");
            hostport = format!("{}:{}", host, port).to_string();
            album::build_alben(
                &app_state.base_path,
                &app_state.single_album,
                &app_state.filtered_extensions,
                &app_state.store,
            );
            open_browser = open;
        }
    }

    if &app_state.prefix != "/" {
        println!("Prefix: {}", &app_state.prefix);
    }

    // let base_path = StdPath::new(&app_state.base_path);
    // print!("Base path: {:#?}\n", base_path.file_name().unwrap());

    // let serve_dir = ServeDir::new("public");
    // let serve_assets = ServeEmbed::<Assets>::new();

    // setup our application with "hello world" route at "/
    // let mut app = Router::new(); Router<Arc<AppState>>

    let router = Router::new()
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

    // let prefixed_router = Router::new().nest(&http_prefix, app);

    // cfg.prefix.unwrap_or("/")
    let router = match String::from(http_prefix).as_str() {
        "/" | "" => router,
        http_prefix => Router::new().nest(&http_prefix, router),
    };

    // start the server
    let listener = tokio::net::TcpListener::bind(hostport.clone())
        .await
        .unwrap();

    println!("Listening on http://{}", hostport);

    if open_browser {
        // let _ = after_start().await;
        let future = after_start(hostport.clone());
        set_timeout_async!(future, 600);
    }
    axum::serve(listener, router).await.unwrap();
}

async fn after_start(hostport: String) {
    println!("Opening Webbrowser");
    webbrowser::open(&format!("http://{}", hostport)).unwrap();
}

async fn if_single_album_redirect(
    State(app_state): State<Arc<AppState>>,
) -> impl axum::response::IntoResponse {
    if app_state.single_album != "" {
        Redirect::permanent(&format!("{}{}", app_state.prefix, app_state.single_album))
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
