use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_source_vo::ContentString;
use shared::common::utility_file_handler as utility_file;
use shared::common::utility_file_handler;

// PURPOSE: FileAdapter — capabilities layer for file I/O operations
use shared::auto_fix::contract_file_adapter_protocol::IFileAdapterProtocol;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct FileAdapter;

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IFileAdapterProtocol for FileAdapter {
    fn read_file(&self, path: &FilePath) -> Option<ContentString> {
        if !utility_file_handler::path_exists(&path.value) {
            return None;
        }
        utility_file_handler::read_file_generic(&path.value)
            .ok()
            .map(ContentString::new)
    }

    fn write_file(&self, path: &FilePath, content: &ContentString) -> bool {
        utility_file_handler::write_file(&path.value, &content.value).is_ok()
    }

    fn path_exists(&self, path: &FilePath) -> bool {
        utility_file_handler::path_exists(&path.value)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for FileAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl FileAdapter {
    pub fn new() -> Self {
        Self
    }
}
