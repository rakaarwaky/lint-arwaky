
use shared::common::taxonomy_path_vo::FilePath;
use shared::git_hooks::contract_git_file_check_protocol::IGitFileCheckProtocol;

// PURPOSE: FileSystemCheckAdapter — IGitFileCheckProtocol implementation for file/directory existence checks
use std::path::Path;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct FileSystemCheckAdapter;

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait::async_trait]
impl IGitFileCheckProtocol for FileSystemCheckAdapter {
    async fn path_exists(&self, path: &FilePath) -> bool {
        Path::new(path.value()).exists()
    }

    async fn is_file(&self, path: &FilePath) -> bool {
        Path::new(path.value()).is_file()
    }

    async fn is_dir(&self, path: &FilePath) -> bool {
        Path::new(path.value()).is_dir()
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl FileSystemCheckAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileSystemCheckAdapter {
    fn default() -> Self {
        Self::new()
    }
}

