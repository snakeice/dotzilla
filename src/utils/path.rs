use home::home_dir;
use std::{fs, path::PathBuf};

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

/// Compact path to a relative path from home directory
/// e.g. /home/user/.config -> ~/.config
pub fn reduce_path_to_home(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        return PathBuf::from(path);
    }

    let path = get_full_path(path).to_string_lossy().to_string();

    if let Some(home) = home_dir() {
        if let Some(home_str) = home.to_str() {
            if path.starts_with(home_str) {
                return PathBuf::from(path.replacen(home_str, "~", 1));
            }
        }
    }

    PathBuf::from(path)
}

/// Get full path of a file in the current working directory
pub fn get_full_path(path: &str) -> PathBuf {
    let p = expand_tilde(path);
    fs::canonicalize(path).unwrap_or_else(|_| {
        if p.is_relative() {
            let current_dir = std::env::current_dir().unwrap();
            if path.starts_with("./") {
                return current_dir.join(path.trim_start_matches("./"));
            }

            current_dir.join(path)
        } else {
            p
        }
    })
}

/// Replace the home directory to custom path
pub fn replace_home(path: &str, new_home: &str) -> PathBuf {
    let path = get_full_path(path).to_string_lossy().to_string();

    if path.starts_with("~") {
        return PathBuf::from(path.replacen("~", new_home, 1));
    }

    if let Some(home) = home_dir() {
        if let Some(home_str) = home.to_str() {
            if path.starts_with(home_str) {
                return PathBuf::from(path.replacen(home_str, new_home, 1));
            }
        }
    }

    PathBuf::from(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_tilde() {
        let path = "~/";
        let expanded_path = expand_tilde(path);
        assert!(expanded_path.exists());
    }

    #[test]
    fn test_get_full_path() {
        let current_dir = std::env::current_dir().unwrap();
        let files = fs::read_dir(current_dir).unwrap();
        for file in files {
            let file = file.unwrap();
            let file_name = "./".to_string() + file.file_name().to_str().unwrap();
            let full_path = get_full_path(&file_name);
            assert!(full_path.exists());
            println!(
                "File: {:?} -> {:?}",
                file_name,
                full_path
            );
        }
    }

    #[test]
    fn test_compact_path() {
        let current_dir = std::env::current_dir().unwrap();
        let files = fs::read_dir(current_dir).unwrap();
        for file in files {
            let file = file.unwrap();
            let full_path = get_full_path(file.file_name().to_str().unwrap());
            let compacted_path = reduce_path_to_home(full_path.to_str().unwrap());
            println!(
                "File: {:?} -> {:?}",
                file.file_name().to_str().unwrap(),
                compacted_path
            );
            assert_ne!(compacted_path, full_path);
        }
    }
}
