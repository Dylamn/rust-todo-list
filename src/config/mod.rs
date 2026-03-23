mod loader;
pub mod logging;

use log::debug;
use std::path::PathBuf;

const ENV_TASK_FILE: &str = "TASK_FILE";

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
    if let Ok(env_path) = std::env::var(ENV_TASK_FILE) {
        debug!("Environment variable `{}` found.", ENV_TASK_FILE);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_should_override_cli() {
        unsafe {
            std::env::set_var(ENV_TASK_FILE, "/env/path");
        }

        let result = resolve_path(Some("/cli/path".into()));

        assert_eq!(result, PathBuf::from("/env/path"));

        unsafe {
            std::env::remove_var(ENV_TASK_FILE);
        }
    }

    #[test]
    fn cli_should_override_config() {
        let result = resolve_path(Some("/cli/path".into()));

        assert_eq!(result, PathBuf::from("/cli/path"));
    }
}
