use home::home_dir;
use std::path::PathBuf;

/// Expand tilde to home directory in path
pub fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with("~") {
        if let Some(home) = home_dir() {
            if let Some(home_str) = home.to_str() {
                return PathBuf::from(path.replacen("~", home_str, 1));
            }
        }
    }
    PathBuf::from(path)
}
