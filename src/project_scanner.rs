use std::fs;
use std::path::{Path, PathBuf};

/// Recursively scans a directory for Rust source files (.rs)
pub fn scan_rust_project<P: AsRef<Path>>(dir: P) -> Vec<PathBuf> {
    let mut rust_files = Vec::new();
    visit_dir(dir.as_ref(), &mut rust_files);
    rust_files
}

fn visit_dir(dir: &Path, rust_files: &mut Vec<PathBuf>) {
    if !dir.is_dir() {
        return;
    }

    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            visit_dir(&path, rust_files);
        } else if let Some(ext) = path.extension() {
            if ext == "rs" {
                rust_files.push(path);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_scan_current_dir() {
        let current_dir = env::current_dir().unwrap();
        let files = scan_rust_project(current_dir);
        assert!(files.iter().any(|f| f.extension().unwrap_or_default() == "rs"));
    }
}
