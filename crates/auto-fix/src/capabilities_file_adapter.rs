// PURPOSE: FileAdapter — capabilities layer for file I/O operations
//
// Wraps IFilesystemAggregate behind IFileAdapterProtocol so that
// auto-fix consumers never depend on std::fs directly.

use shared::auto_fix::contract_fix_protocol::IFileAdapterProtocol;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_source_vo::ContentString;
use shared::filesystem::FilesystemRequest;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct FileAdapter {
    filesystem: Arc<dyn IFilesystemAggregate>,
    io: Arc<dyn IFileSystemIOProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IFileAdapterProtocol for FileAdapter {
    fn read_file(&self, path: &FilePath) -> Option<ContentString> {
        if !self.io.path_exists(std::path::Path::new(path.value())) {
            return None;
        }
        self.filesystem
            .execute(FilesystemRequest::read_file(std::path::Path::new(
                path.value(),
            )))
            .into_content_opt()
            .map(ContentString::new)
    }

    fn write_file(&self, path: &FilePath, content: &ContentString) -> bool {
        self.io
            .write_string(std::path::Path::new(path.value()), &content.value)
            .is_ok()
    }

    fn path_exists(&self, path: &FilePath) -> bool {
        self.io.path_exists(std::path::Path::new(path.value()))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl FileAdapter {
    pub fn new(
        filesystem: Arc<dyn IFilesystemAggregate>,
        io: Arc<dyn IFileSystemIOProtocol>,
    ) -> Self {
        Self { filesystem, io }
    }
}
