# Examples

## BAD: Infrastructure Without Port (AES404)

```rust
pub struct FileCache;

impl FileCache {
    pub fn read(&self) {
        // public behavior without port trait
    }
}
```

Fix:

```rust
pub struct FileCache;

impl IFileCachePort for FileCache {
    // contract implementation
}
```

## BAD: Business Logic in Infrastructure

```rust
impl OrphanFileCache {
    fn analyze(&self, content: &FileContent) -> bool {
        // BAD: domain logic
        content.value().contains("orphan")
    }
}
```

Fix: Move analysis to capabilities.

## BAD: Std Trait in Block 2

```rust
pub struct FileCacheAdapter;

impl Default for FileCacheAdapter {
    fn default() -> Self { Self }
}

impl IFileReaderPort for FileCacheAdapter {
    fn read(&self, path: &FilePath) -> Result<FileContent, FileReadError> { // ...
    }
}
```

Fix: Move `Default` to Block 3.

## GOOD: Correct 3-Block Order

```rust
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

    fn is_not_found(err: &std::io::Error) -> bool {
        err.kind() == std::io::ErrorKind::NotFound
    }
}
```
