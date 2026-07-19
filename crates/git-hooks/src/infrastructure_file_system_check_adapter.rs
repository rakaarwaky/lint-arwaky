// PURPOSE: FileSystemCheckAdapter — IGitFileCheckPort implementation for file/directory existence checks
use std::path::Path;

use shared::git_hooks::contract_git_file_check_port::IGitFileCheckPort;

// Block 1: struct Definition
pub struct FileSystemCheckAdapter;

// Block 2: impl Port for Struct (Public Contract)
#[async_trait::async_trait]
impl IGitFileCheckPort for FileSystemCheckAdapter {
    async fn path_exists(&self, path: &str) -> bool {
        Path::new(path).exists()
    }

    async fn is_file(&self, path: &str) -> bool {
        Path::new(path).is_file()
    }

    async fn is_dir(&self, path: &str) -> bool {
        Path::new(path).is_dir()
    }
}

// Block 3: constructors
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
