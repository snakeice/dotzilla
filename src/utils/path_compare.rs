use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

pub enum CompareResult {
    Equal,
    NotEqual,
    Linked,
}

pub fn compare_file(a: &Path, b: &Path) -> Result<bool, Box<dyn std::error::Error>> {
    let file_a = File::open(a)?;
    let file_b = File::open(b)?;

    if file_a.metadata()?.len() != file_b.metadata()?.len() {
        return Ok(false);
    }

    let mut reader_a = BufReader::new(file_a);
    let mut reader_b = BufReader::new(file_b);

    let mut buffer_a = [0; 4096];
    let mut buffer_b = [0; 4096];

    loop {
        let n_a = reader_a.read(&mut buffer_a)?;
        let n_b = reader_b.read(&mut buffer_b)?;

        if n_a != n_b || buffer_a[..n_a] != buffer_b[..n_b] {
            return Ok(false);
        }

        if n_a == 0 {
            break;
        }
    }
    Ok(true)
}

fn walk_dir(path: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut paths = Vec::new();
    for entry in path.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            paths.extend(walk_dir(&path)?);
        } else {
            paths.push(path);
        }
    }
    Ok(paths)
}

pub fn linked_paths(a: &Path, b: &Path) -> Result<bool, Box<dyn std::error::Error>> {
    if a.is_symlink() && b.is_symlink() {
        let target_a = fs::read_link(a)?;
        let target_b = fs::read_link(b)?;
        return Ok(target_a == target_b);
    }
    if a.is_symlink() {
        let target_a = fs::read_link(a)?;
        return Ok(target_a == b);
    }
    if b.is_symlink() {
        let target_b = fs::read_link(b)?;
        return Ok(a == target_b);
    }
    Ok(false)
}

pub fn compare_paths(a: &Path, b: &Path) -> Result<CompareResult, Box<dyn std::error::Error>> {
    if a.exists() != b.exists() || a.is_dir() != b.is_dir() {
        return Ok(CompareResult::NotEqual);
    }

    if linked_paths(a, b)? {
        return Ok(CompareResult::Linked);
    }

    if !a.is_dir() && !b.is_dir() {
        match compare_file(a, b) {
            Ok(true) => return Ok(CompareResult::Equal),
            Ok(false) => return Ok(CompareResult::NotEqual),
            Err(e) => return Err(e),
        }
    }

    let paths_a = walk_dir(a)?;
    let paths_b = walk_dir(b)?;
    if paths_a.len() != paths_b.len() {
        return Ok(CompareResult::NotEqual);
    }

    let mut processed_paths = Vec::new();
    for path_b in &paths_b {
        let relative_path = path_b.strip_prefix(b)?;
        if processed_paths.contains(&relative_path) {
            continue;
        }
        let corresponding_path_a = a.join(relative_path);
        if !corresponding_path_a.exists() {
            return Ok(CompareResult::NotEqual);
        }
        processed_paths.push(relative_path);
    }

    for path_a in &paths_a {
        let relative_path = path_a.strip_prefix(a)?;
        if processed_paths.contains(&relative_path) {
            continue;
        }
        let corresponding_path_b = b.join(relative_path);
        if !corresponding_path_b.exists() {
            return Ok(CompareResult::NotEqual);
        }
        processed_paths.push(relative_path);
    }

    for path in processed_paths {
        let corresponding_path_a = a.join(path);
        let corresponding_path_b = b.join(path);
        if !compare_file(&corresponding_path_a, &corresponding_path_b)? {
            return Ok(CompareResult::NotEqual);
        }
    }

    Ok(CompareResult::Equal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq_file() {
        let tmp_file_a = Path::new("/tmp/test_a.txt");
        let tmp_file_b = Path::new("/tmp/test_b.txt");
        let content = b"Hello, world!";

        std::fs::write(tmp_file_a, content).unwrap();
        std::fs::write(tmp_file_b, content).unwrap();

        match compare_paths(tmp_file_a, tmp_file_b) {
            Ok(CompareResult::Equal) => println!("Files are equal"),
            Ok(CompareResult::NotEqual) => println!("Files are not equal"),
            Ok(CompareResult::Linked) => println!("Files are linked"),
            Err(e) => panic!("Error comparing files: {}", e),
        }
    }

    #[test]
    fn test_eq_dir() {
        let tmp_dir_a = Path::new("/tmp/test_dir_a");
        let tmp_dir_b = Path::new("/tmp/test_dir_b");

        std::fs::create_dir_all(tmp_dir_a).unwrap();
        std::fs::create_dir_all(tmp_dir_b).unwrap();

        let content = b"Hello, world!";
        std::fs::write(tmp_dir_a.join("file.txt"), content).unwrap();
        std::fs::write(tmp_dir_b.join("file.txt"), content).unwrap();

        match compare_paths(tmp_dir_a, tmp_dir_b) {
            Ok(CompareResult::Equal) => println!("Files are equal"),
            Ok(CompareResult::NotEqual) => println!("Files are not equal"),
            Ok(CompareResult::Linked) => println!("Files are linked"),
            Err(e) => panic!("Error comparing directories: {}", e),
        }
    }
}
