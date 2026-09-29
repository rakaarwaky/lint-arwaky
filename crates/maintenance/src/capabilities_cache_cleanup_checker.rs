use shared::common::taxonomy_path_vo::FilePath;
use shared::filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared::maintenance::contract_maintenance_protocol::ICacheCleanupProtocol;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct CacheCleanupChecker {
    io: Arc<dyn IFileSystemIOProtocol>,
    project_path: FilePath,
}

// ─── Block 2: Protocol Implementation ─────────────────────
impl ICacheCleanupProtocol for CacheCleanupChecker {
    fn clean(&self) {
        let base = &self.project_path.value;
        for dir in &[
            ".pytest_cache",
            "__pycache__",
            "node_modules/.cache",
            "target",
        ] {
            let path = FilePath::new(format!("{base}/{dir}")).unwrap_or_default();
            if path.value.is_empty() {
                continue;
            }
            let _ = self.io.remove_dir_all(std::path::Path::new(&path.value));
        }
    }
}

// ─── Block 3: Constructors & Helpers ──────────────────────
impl CacheCleanupChecker {
    pub fn new(io: Arc<dyn IFileSystemIOProtocol>) -> Self {
        Self {
            io,
            project_path: FilePath::new(".").unwrap_or_default(),
        }
    }
}
