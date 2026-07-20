// PURPOSE: Orphan file cache utility — stateless interface to thread-local file cache
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;

thread_local! {
    static FILE_CACHE: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

pub fn read_cached(path: &FilePath) -> ContentString {
    FILE_CACHE.with(|cache| -> ContentString {
        let mut cache = cache.borrow_mut();
        if let Some(content) = cache.get(path.value()) {
            return ContentString::new(content.clone());
        }
        let content = fs::read_to_string(path.value()).unwrap_or_default();
        cache.insert(path.value().to_string(), content.clone());
        ContentString::new(content)
    })
}

pub fn read_dir(dir_path: &FilePath) -> Vec<FilePath> {
    let mut entries = Vec::new();
    if let Ok(read_dir) = fs::read_dir(dir_path.value()) {
        for entry in read_dir.flatten() {
            if let Some(s) = entry.path().to_str() {
                if let Ok(fp) = FilePath::new(s) {
                    entries.push(fp);
                }
            }
        }
    }
    entries
}

pub fn path_exists(path: &FilePath) -> bool {
    std::path::Path::new(path.value()).exists()
}

pub fn is_symlink(path: &FilePath) -> bool {
    std::fs::symlink_metadata(path.value())
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false)
}

pub fn clear_cache() {
    FILE_CACHE.with(|c| c.borrow_mut().clear());
}
