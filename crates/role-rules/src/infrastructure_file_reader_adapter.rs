// PURPOSE: FileReaderAdapter — infrastructure layer for file reading operations
use shared::role_rules::contract_file_reader_port::IFileReaderPort;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct FileReaderAdapter;

// ─── Block 2: Public Contract (domain port ONLY) ──────────
impl IFileReaderPort for FileReaderAdapter {
    fn read_file(&self, path: &str) -> Option<String> {
        std::fs::read_to_string(path).ok()
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl FileReaderAdapter {
    pub fn new() -> Self {
        Self
    }
}
