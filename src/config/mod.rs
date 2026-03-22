mod loader;

use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct Config {
    pub file_path: Option<PathBuf>,
}

/// Priority:
/// - Env var
/// - CLI argument
/// - Config file
/// - Default location
pub fn resolve_path(cli_path: Option<PathBuf>) -> PathBuf {
    if let Ok(env_path) = std::env::var("TASK_FILE") {
        return PathBuf::from(env_path);
    }

    if let Some(path) = cli_path {
        return path;
    }

    if let Some(file_config) = loader::load_from_file() {
        if let Some(path) = file_config.file_path {
            return path;
        }
    }

    default_storage_path()
}

fn default_storage_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("No home dir");
    path.push(".task-cli");
    std::fs::create_dir_all(&path).ok();
    path.push("tasks.json");
    path
}