use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_source_vo::ContentString;
use shared::orphan_detector::contract_orphan_protocol::IOrphanFileCachePort;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;

thread_local! {
    static FILE_CACHE: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

// ─── Block 1: Struct Definition ───────────────────────────
pub struct OrphanFileCache;

// ─── Block 2: Public Contract ─────────────────────────────
impl Default for OrphanFileCache {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Block 3: Constructors & Helpers ──────────────────────
impl OrphanFileCache {
    pub fn new() -> Self {
        Self
    }
}

impl IOrphanFileCachePort for OrphanFileCache {
    fn read_cached(&self, path: &FilePath) -> ContentString {
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

    fn clear_cache(&self) {
        FILE_CACHE.with(|c| c.borrow_mut().clear());
    }
}
