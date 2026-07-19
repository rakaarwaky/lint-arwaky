// PURPOSE: FileAdapter — infrastructure layer for file I/O operations
use shared::auto_fix::contract_file_adapter_port::IFileAdapterPort;
use std::path::Path;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct FileAdapter;

// ─── Block 2: Public Contract (domain port ONLY) ──────────
impl IFileAdapterPort for FileAdapter {
    fn read_file(&self, path: &str) -> Option<String> {
        let p = Path::new(path);
        if !p.exists() {
            return None;
        }
        std::fs::read_to_string(p).ok()
    }

    fn write_file(&self, path: &str, content: &str) -> bool {
        std::fs::write(path, content).is_ok()
    }

    fn path_exists(&self, path: &str) -> bool {
        Path::new(path).exists()
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl FileAdapter {
    pub fn new() -> Self {
        Self
    }
}
