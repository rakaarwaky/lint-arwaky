use shared::auto_fix::utility_auto_fix_io as af_io;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_source_vo::ContentString;

// PURPOSE: FileAdapter — infrastructure layer for file I/O operations
use shared::auto_fix::contract_file_adapter_protocol::IFileAdapterProtocol;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct FileAdapter;

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IFileAdapterProtocol for FileAdapter {
    fn read_file(&self, path: &FilePath) -> Option<ContentString> {
        if !af_io::path_exists(&path.value) {
            return None;
        }
        af_io::read_file(&path.value).map(ContentString::new)
    }

    fn write_file(&self, path: &FilePath, content: &ContentString) -> bool {
        af_io::write_file(&path.value, &content.value)
    }

    fn path_exists(&self, path: &FilePath) -> bool {
        af_io::path_exists(&path.value)
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

