use shared::common::{ContentString, FilePath};

// PURPOSE: FileAdapter — capabilities layer for file I/O operations
use shared::auto_fix::IFileAdapterProtocol;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct FileAdapter;

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IFileAdapterProtocol for FileAdapter {
    fn read_file(&self, path: &FilePath) -> Option<ContentString> {
        if !filesystem::utility_io::path_exists(&path.value) {
            return None;
        }
        filesystem::utility_io::read_file(&path.value)
            .ok()
            .map(ContentString::new)
    }

    fn write_file(&self, path: &FilePath, content: &ContentString) -> bool {
        filesystem::utility_io::write_file(&path.value, &content.value).is_ok()
    }

    fn path_exists(&self, path: &FilePath) -> bool {
        filesystem::utility_io::path_exists(&path.value)
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
