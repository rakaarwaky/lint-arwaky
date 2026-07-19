use shared::common::taxonomy_common_vo::BooleanVO;
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

// ─── Block 2: Public Contract (domain port ONLY) ──────────
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

    fn read_dir(&self, dir_path: &FilePath) -> Vec<FilePath> {
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

    fn path_exists(&self, path: &FilePath) -> BooleanVO {
        BooleanVO::from(std::path::Path::new(path.value()).exists())
    }

    fn is_symlink(&self, path: &FilePath) -> BooleanVO {
        BooleanVO::from(
            std::fs::symlink_metadata(path.value())
                .map(|m| m.file_type().is_symlink())
                .unwrap_or(false),
        )
    }

    fn clear_cache(&self) {
        FILE_CACHE.with(|c| c.borrow_mut().clear());
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl Default for OrphanFileCache {
    fn default() -> Self {
        Self::new()
    }
}

impl OrphanFileCache {
    pub fn new() -> Self {
        Self
    }
}
