#[derive(Serialize, Deserialize)]
struct Greeting {
    greeting: String,
    visitor: String,
    visits: u16,
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

// number_of_visits: AtomicU16,
/*
.route("/hello/{visitor}", get(greet_visitor))
        .route("/bye", delete(say_goodbye))
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

pub fn list_files_paths(base: &str, name: &str) -> Vec<PathBuf> {
    let pattern = format!("{}/{}/", base, name);
    print!("Listing files without info in pattern: {}\n", pattern);
    let files: Vec<PathBuf> = fs::read_dir(&pattern)
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

    return files;
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

async fn resize_image2old(
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
        .write_to(&mut buffer, ImageFormat::Jpeg)
        .unwrap();

    let bytes: Vec<u8> = buffer.into_inner().unwrap().into_inner();

    (
        axum::response::AppendHeaders([(header::CONTENT_TYPE, "image/jpg")]),
        bytes,
    )
}

pub fn build_if_needed0(base: &str, name: &str, filtered_extensions: &Vec<String>) -> Album {
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

cli::Commands::Stats { host, port } => {
            print!("Fetching stats from server at {}:{}\n", host, port);
            match fetch_stats(host, port) {
                Ok(stats) => {
                    println!("{:#?}", stats);
                }
                Err(e) => println!("Error: {}", e),
            }
            std::process::exit(0);
        }

fn fetch_stats(host: String, port: u16) -> Result<Stats, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(format!("http://{}:{}/stats", host, port))?;
    if resp.status().is_success() {
        let stats: Stats = resp.json()?;
        Ok(stats)
    } else {
        Err(format!("Server returned status: {}", resp.status()).into())
    }
}