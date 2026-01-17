use std::path::PathBuf;

pub struct Store {
    pub base_path: PathBuf,
    pub cache_path: PathBuf,
}

impl Store {
    pub fn new(base: &str) -> Self {
        let base_path = PathBuf::from(base);
        let cache_path = base_path.join("cache");
        std::fs::create_dir_all(&cache_path).unwrap();
        Store {
            base_path,
            cache_path,
        }
    }

    pub fn get_album_index(&self, album_path: String) -> Option<String> {
        let index_path = self
            .base_path
            .join(id(&PathBuf::from(&album_path)) + ".json");
        if index_path.exists() {
            let data = std::fs::read_to_string(index_path).ok()?;
            // let images: Vec<String> = serde_json::from_str(&data).ok()?;
            Some(data)
        } else {
            None
        }
    }

    pub fn save_album_index(&self, album_path: &str, data: &str) {
        let album_cache_path = self
            .base_path
            .join(id(&PathBuf::from(album_path)) + ".json");
        // std::fs::create_dir_all(&album_cache_path).unwrap();
        std::fs::write(album_cache_path, data).unwrap();
    }
}

fn id(path: &PathBuf) -> String {
    format!(
        "{:x}-{}",
        md5::compute(path.to_string_lossy().to_string()),
        path.file_name().unwrap().to_str().unwrap().to_string()
    )
}
