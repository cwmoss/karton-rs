use serde::{self, Serialize};
use serde_json::json;
use std::{fs, io, path::PathBuf};

#[derive(Serialize)]
struct Dirs {
    name: String,
    items: Vec<PathBuf>,
}

fn main() -> io::Result<()> {
    let path = PathBuf::from(".").canonicalize().unwrap();
    let mut entries: Vec<PathBuf> = fs::read_dir(path)?
        // .unwrap()
        .filter_map(|res| {
            let entry = res.ok()?;
            let path = entry.path();
            if path.file_name()?.to_str()?.starts_with(".") {
                return None;
            }
            Some(path)
        })
        .collect();

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    entries.sort();

    dbg!(&entries);

    print!(
        "json: {}",
        serde_json::to_string(&Dirs {
            name: "test".to_string(),
            items: entries
        })
        .unwrap()
    );

    // The entries have now been sorted by their path.

    Ok(())
}
