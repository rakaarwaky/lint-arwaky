use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_source_vo::ContentString;
use std::path::Path;

// PURPOSE: FileAdapter — infrastructure layer for file I/O operations
use shared::auto_fix::contract_file_adapter_protocol::IFileAdapterProtocol;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct FileAdapter;

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IFileAdapterProtocol for FileAdapter {
    fn read_file(&self, path: &FilePath) -> Option<ContentString> {
        let p = Path::new(path.value());
        if !p.exists() {
            return None;
        }
        std::fs::read_to_string(p).ok().map(ContentString::new)
    }

    fn write_file(&self, path: &FilePath, content: &ContentString) -> bool {
        std::fs::write(path.value(), &content.value).is_ok()
    }

    fn path_exists(&self, path: &FilePath) -> bool {
        Path::new(path.value()).exists()
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

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

