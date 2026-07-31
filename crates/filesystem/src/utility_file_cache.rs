// PURPOSE: Utility layer — file content cache with DashMap
// Read once, serve from memory. Thread-safe parallel population.

use crate::contract_filesystem_protocol::IFileCacheProtocol;
use crate::taxonomy_filesystem_vo::*;
use camino::Utf8PathBuf;
use dashmap::DashMap;
use rayon::prelude::*;
use std::sync::Arc;

/// Thread-safe file content cache.
pub struct FileCache {
    cache: Arc<DashMap<Utf8PathBuf, String>>,
}

impl FileCache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(DashMap::new()),
        }
    }

    /// Memory usage estimate (bytes).
    pub fn memory_bytes(&self) -> usize {
        self.cache
            .iter()
            .map(|entry| entry.key().as_str().len() + entry.value().len())
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
            let content = match std::fs::read_to_string(entry.path.as_std_path()) {
                Ok(c) => c,
                Err(_) => return,
            };
            cache.insert(entry.path.clone(), content);
        });
    }

    fn get(&self, path: &Utf8PathBuf) -> Option<String> {
        self.cache.get(path).map(|r| r.value().clone())
    }

    fn contains(&self, path: &Utf8PathBuf) -> bool {
        self.cache.contains_key(path)
    }
}
