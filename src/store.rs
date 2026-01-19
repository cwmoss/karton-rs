use std::path::PathBuf;

use crate::album_image::Size;

#[derive(Clone)]
pub struct Store {
    pub base_path: PathBuf,
    pub cache_path: PathBuf,
}

pub enum ImageFile {
    Found {
        path: PathBuf,
        // file_stream: std::fs::File,
    },
    NotFound {
        path: PathBuf,
    },
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

    pub fn get_image_cache_path(&self, album_path: &str, img: &str, size: Size) -> PathBuf {
        let name = format!(
            "{}-{}__{}x{}.jpg",
            id(&PathBuf::from(album_path)),
            img,
            size.0,
            size.1
        );
        let album_cache_path = self.cache_path.join(name);
        album_cache_path
    }

    pub fn image_exists_in_cache(&self, album_path: &str, img: &str, size: Size) -> ImageFile {
        let img_cache_path = self.get_image_cache_path(album_path, img, size);
        if img_cache_path.exists() {
            ImageFile::Found {
                path: img_cache_path,
            }
        } else {
            ImageFile::NotFound {
                path: img_cache_path,
            }
        }
    }

    pub fn get_admin_secret(&self) -> Option<String> {
        std::fs::read_to_string(self.base_path.join(".admin_secret")).ok()
    }

    pub fn save_admin_secret(&self, secret: String) -> String {
        std::fs::write(self.base_path.join(".admin_secret"), secret.clone()).unwrap();
        secret
    }

    pub fn clear_cache(&self) {
        if self.cache_path.exists() {
            std::fs::remove_dir_all(&self.cache_path).unwrap();
        }
        std::fs::create_dir_all(&self.cache_path).unwrap();
    }
}

fn id(path: &PathBuf) -> String {
    format!(
        "{:x}-{}",
        md5::compute(path.to_string_lossy().to_string()),
        path.file_name().unwrap().to_str().unwrap().to_string()
    )
}
