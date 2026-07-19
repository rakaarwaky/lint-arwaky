// PURPOSE: FileAdapter — infrastructure layer for file I/O operations
use shared::auto_fix::contract_file_adapter_port::IFileAdapterPort;
use shared::common::taxonomy_path_vo::FilePath;
use std::path::Path;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct FileAdapter;

// ─── Block 2: Public Contract (domain port ONLY) ──────────
impl IFileAdapterPort for FileAdapter {
    fn read_file(&self, path: &FilePath) -> Option<String> {
        let p = Path::new(path.value());
        if !p.exists() {
            return None;
        }
        std::fs::read_to_string(p).ok()
    }

    fn write_file(&self, path: &FilePath, content: &str) -> bool {
        std::fs::write(path.value(), content).is_ok()
    }

    fn path_exists(&self, path: &FilePath) -> bool {
        Path::new(path.value()).exists()
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl FileAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileAdapter {
    fn default() -> Self {
        Self::new()
    }
}
