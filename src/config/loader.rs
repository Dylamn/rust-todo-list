use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct FileConfig {
    pub file_path: Option<PathBuf>,
}

pub fn load_from_file() -> Option<FileConfig> {
    let mut path = dirs::home_dir()?;
    path.push(".task-cli");
    path.push("config.toml");

    let content = fs::read_to_string(path).ok()?;

    toml::from_str(&content).ok()
}