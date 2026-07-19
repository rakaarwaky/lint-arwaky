use std::sync::Arc;

use shared::file_system::taxonomy_file_content_vo::FileContent;
use shared::file_system::taxonomy_file_path_vo::FilePath;
use shared::file_system::taxonomy_file_read_error::FileReadError;
use shared::file_system::contract_file_reader_port::IFileReaderPort;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct FileSystemSourceReader;

// ─── Block 2: Public Contract (domain port ONLY) ──────────
impl IFileReaderPort for FileSystemSourceReader {
    fn read(&self, path: &FilePath) -> Result<FileContent, FileReadError> {
        let raw = std::fs::read_to_string(path.value())
            .map_err(|err| FileReadError::io(path.clone(), err))?;
        FileContent::new(raw)
            .map_err(FileReadError::validation)
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl Default for FileSystemSourceReader {
    fn default() -> Self { Self }
}

impl FileSystemSourceReader {
    pub fn new() -> Self { Self }
}
