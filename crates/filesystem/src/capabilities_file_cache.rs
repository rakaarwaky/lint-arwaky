// PURPOSE: Capabilities layer — file content cache (FR-002)
// Read once, serve from memory. Thread-safe parallel population. — file content cache with DashMap
// Read once, serve from memory. Thread-safe parallel population.

use dashmap::DashMap;
use rayon::prelude::*;
use shared::filesystem::IFileCacheProtocol;
use shared::filesystem::taxonomy_filesystem_vo::*;
use std::path::PathBuf;
use std::sync::Arc;

pub struct FileCache {
    cache: Arc<DashMap<PathBuf, String>>,
}

impl FileCache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(DashMap::new()),
        }
    }

    pub fn memory_bytes(&self) -> usize {
        self.cache
            .iter()
            .map(|e| e.key().as_os_str().len() + e.value().len())
            .sum()
    }
}

impl Default for FileCache {
    fn default() -> Self {
        Self::new()
    }
}

impl IFileCacheProtocol for FileCache {
    fn populate(&self, files: &[FileEntry]) {
        let cache = self.cache.clone();
        files.par_iter().for_each(|entry| {
            let content = match std::fs::read_to_string(&entry.path) {
                Ok(c) => c,
                Err(_) => return,
            };
            cache.insert(entry.path.clone(), content);
        });
    }

    fn get(&self, path: &PathBuf) -> Option<String> {
        self.cache.get(path).map(|r| r.value().clone())
    }

    fn contains(&self, path: &PathBuf) -> bool {
        self.cache.contains_key(path)
    }
}
