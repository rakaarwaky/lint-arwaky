// PURPOSE: Filesystem checker utility — stateless filesystem operations for project setup and maintenance
use crate::common::taxonomy_path_vo::FilePath;
use std::fs;
use std::path::Path;

pub fn read_file(path: &FilePath) -> Result<String, String> {
    fs::read_to_string(path.value()).map_err(|e| e.to_string())
}

pub fn write_file(path: &FilePath, content: &str) -> Result<(), String> {
    fs::write(path.value(), content).map_err(|e| e.to_string())
}

pub fn create_dir_all(path: &FilePath) -> Result<(), String> {
    fs::create_dir_all(path.value()).map_err(|e| e.to_string())
}

pub fn path_exists(path: &FilePath) -> bool {
    Path::new(path.value()).exists()
}

pub fn file_exists(path: &FilePath) -> bool {
    let p = Path::new(path.value());
    p.exists() && p.is_file()
}

pub fn walk_py_files(dir: &FilePath) -> Vec<FilePath> {
    let mut files = Vec::new();
    walk_py_files_inner(Path::new(dir.value()), &mut files);
    files
}

pub fn find_cache_dirs(dir: &FilePath, cache_names: &[&str]) -> Vec<FilePath> {
    let mut found = Vec::new();
    find_cache_dirs_inner(Path::new(dir.value()), cache_names, &mut found);
    found
}

pub fn remove_dir_all(path: &FilePath) -> Result<(), String> {
    fs::remove_dir_all(path.value()).map_err(|e| e.to_string())
}

fn walk_py_files_inner(dir: &Path, files: &mut Vec<FilePath>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default();
                if name != "target" && name != ".git" && name != "node_modules" && name != ".venv" {
                    walk_py_files_inner(&path, files);
                }
            } else if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("py") {
                if let Ok(fp) = FilePath::new(path.to_string_lossy().to_string()) {
                    files.push(fp);
                }
            }
        }
    }
}

fn find_cache_dirs_inner(dir: &Path, cache_names: &[&str], found: &mut Vec<FilePath>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default();
                if cache_names.contains(&name) {
                    if let Ok(fp) = FilePath::new(path.to_string_lossy().to_string()) {
                        found.push(fp);
                    }
                } else if name != "target" && name != ".git" && name != "node_modules" {
                    find_cache_dirs_inner(&path, cache_names, found);
                }
            }
        }
    }
}
