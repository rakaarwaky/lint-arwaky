// PURPOSE: FileAdapter — capabilities layer for file I/O operations
//
// Wraps IFilesystemAggregate behind IFileAdapterProtocol so that
// auto-fix consumers never depend on std::fs directly.

use shared::auto_fix::IFileAdapterProtocol;
use shared::common::{ContentString, FilePath};
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct FileAdapter {
    filesystem: Arc<dyn IFilesystemAggregate>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IFileAdapterProtocol for FileAdapter {
    fn read_file(&self, path: &FilePath) -> Option<ContentString> {
        if !self
            .filesystem
            .path_exists(std::path::Path::new(path.value()))
        {
            return None;
        }
        self.filesystem
            .read_file(std::path::Path::new(path.value()))
            .map(ContentString::new)
    }

    fn write_file(&self, path: &FilePath, content: &ContentString) -> bool {
        self.filesystem
            .write_string(std::path::Path::new(path.value()), &content.value)
            .is_ok()
    }

    fn path_exists(&self, path: &FilePath) -> bool {
        self.filesystem
            .path_exists(std::path::Path::new(path.value()))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl FileAdapter {
    pub fn new(filesystem: Arc<dyn IFilesystemAggregate>) -> Self {
        Self { filesystem }
    }
}
