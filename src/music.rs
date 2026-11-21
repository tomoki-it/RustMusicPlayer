use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Music {
    title: String,
    path: String,
}

impl Music {

    pub fn new (title: &str, path: &str) -> Music {
        Music{title: title.to_string(), path: path.to_string()}
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn exist_audio_file(path: &str) -> bool {
        Path::new(path).exists()
    }
}