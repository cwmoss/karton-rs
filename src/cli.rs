use clap::{Parser, Subcommand};
use directories::BaseDirs;
use std::env;
use std::path::Path as StdPath;
use std::path::PathBuf;

/// Karton serves your photo albums over HTTP.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Base path to albums, if not set,
    /// uses current directory or KARTON_BASE env var
    #[arg(short, long, default_value_t=get_default_base_path(), verbatim_doc_comment)]
    pub base: String,

    /// Base path to store, if not set,
    /// uses home directory/.karton  or KARTON_STORE env var
    #[arg(long, default_value_t=get_default_store_path(), verbatim_doc_comment)]
    pub store: String,

    /// Comma-separated list of filtered
    /// extensions (e.g., "jpg,jpeg,png")
    #[arg(long, default_value_t = String::from("jpg,jpeg"), verbatim_doc_comment)]
    pub extensions: String,

    /// Prefix for subpath support
    #[arg(long, default_value_t = String::from("/"))]
    pub prefix: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand, Eq, PartialEq)]
pub enum Commands {
    /// Start the Karton web server
    Serve {
        /// Host to bind to
        #[arg(long, default_value_t = String::from("0.0.0.0"))]
        host: String,

        #[arg(long, default_value_t = 5000)]
        port: u16,

        /// open webbrowser?
        #[arg(short, long, default_value_t = false)]
        open: bool,
    },
    /// Scan for albums and build caches
    Scan {},
}

pub fn get_cli_args_and_setup() -> (Cli, String, String) {
    let args = Cli::parse();
    //let base = args.base.clone();
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

    (args, base, single_album)
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
