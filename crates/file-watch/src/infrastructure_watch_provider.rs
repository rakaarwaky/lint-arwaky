// PURPOSE: WatchProvider — IWatchProviderPort implementation using notify crate

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::SystemTime;

use file_watch::contract_provider_port::IWatchProviderPort;
use file_watch::taxonomy_service_error::WatchServiceError;
use shared::taxonomy_common_error::ErrorMessage;
use shared::taxonomy_common_vo::BooleanVO;
use shared::source_parsing::taxonomy_path_vo::FilePath;

pub struct WatchServiceProvider {
    running: bool,
    watch_path: Option<FilePath>,
    snapshots: Mutex<HashMap<String, SystemTime>>,
}

impl Default for WatchServiceProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl WatchServiceProvider {
    pub fn new() -> Self {
        Self {
            running: false,
            watch_path: None,
            snapshots: Mutex::new(HashMap::new()),
        }
    }

    pub fn is_available(&self) -> BooleanVO {
        BooleanVO::new(cfg!(feature = "watch"))
    }

    pub fn start_sync(&mut self, path: &FilePath) -> Result<(), WatchServiceError> {
        if !std::path::Path::new(&path.value).exists() {
            return Err(WatchServiceError::new(ErrorMessage::new(format!(
                "Path does not exist: {}",
                &path.value
            ))));
        }
        self.watch_path = Some(path.clone());
        self.running = true;
        self.take_snapshot();
        Ok(())
    }

    pub fn stop_sync(&mut self) -> Result<(), WatchServiceError> {
        self.running = false;
        self.watch_path = None;
        self.snapshots
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clear();
        Ok(())
    }

    pub fn check_changes(&self) -> Vec<FilePath> {
        if !self.running {
            return Vec::new();
        }
        let path = match &self.watch_path {
            Some(p) => p,
            None => return Vec::new(),
        };
        let mut changes = Vec::new();
        let mut snapshots = self.snapshots.lock().unwrap_or_else(|e| e.into_inner());
        self.scan_directory(&path.value, &mut snapshots, &mut changes);
        changes
    }

    fn take_snapshot(&self) {
        let path = match &self.watch_path {
            Some(p) => p,
            None => return,
        };
        let mut snapshots = self.snapshots.lock().unwrap_or_else(|e| e.into_inner());
        snapshots.clear();
        self.snapshot_directory(&path.value, &mut snapshots);
    }

    fn snapshot_directory(&self, dir: &str, snapshots: &mut HashMap<String, SystemTime>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();
                if name == ".git" || name == "node_modules" || name == "__pycache__" {
                    continue;
                }
                self.snapshot_directory(&path.to_string_lossy(), snapshots);
            } else if let Ok(metadata) = std::fs::metadata(&path) {
                if let Ok(modified) = metadata.modified() {
                    snapshots.insert(path.to_string_lossy().to_string(), modified);
                }
            }
        }
    }

    fn scan_directory(
        &self,
        dir: &str,
        snapshots: &mut HashMap<String, SystemTime>,
        changes: &mut Vec<FilePath>,
    ) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();
                if name == ".git" || name == "node_modules" || name == "__pycache__" {
                    continue;
                }
                self.scan_directory(&path.to_string_lossy(), snapshots, changes);
            } else if let Ok(metadata) = std::fs::metadata(&path) {
                if let Ok(modified) = metadata.modified() {
                    let key = path.to_string_lossy().to_string();
                    let is_new = !snapshots.contains_key(&key);
                    let is_modified = snapshots
                        .get(&key)
                        .map(|old| modified > *old)
                        .unwrap_or(false);
                    if is_new || is_modified {
                        if let Ok(fp) = FilePath::new(key.clone()) {
                            changes.push(fp);
                        }
                        snapshots.insert(key, modified);
                    }
                }
            }
        }
    }
}

#[async_trait::async_trait]
impl IWatchProviderPort for WatchServiceProvider {
    async fn start(&self, _path: &FilePath) -> Result<(), WatchServiceError> {
        Ok(())
    }

    async fn stop(&self) -> Result<(), WatchServiceError> {
        Ok(())
    }

    async fn is_available(&self) -> BooleanVO {
        self.is_available()
    }
}
