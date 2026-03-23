use log::LevelFilter;

pub fn resolve_log_level(verbose: u8, log_level: Option<String>) -> LevelFilter {
    // Priority to `--log-level`
    if let Some(level) = log_level {
        return match level.to_lowercase().as_str() {
            "error" => LevelFilter::Error,
            "warn" => LevelFilter::Warn,
            "info" => LevelFilter::Info,
            "debug" => LevelFilter::Debug,
            "trace" => LevelFilter::Trace,
            _ => LevelFilter::Info,
        };
    }

    // Fallback on `-v`
    match verbose {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    }
}