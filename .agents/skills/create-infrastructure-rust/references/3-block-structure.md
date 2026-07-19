# The 3-Block Structure

1. **Block 1 — Struct Definition**
2. **Block 2 — Port Trait Implementation**
3. **Block 3 — Constructors, Std Traits, and Private Helpers**

## Block 1 — Struct Definition

```rust
pub struct FileSystemSourceReader;
```

## Block 2 — Public Contract

Block 2 is RESERVED for the domain port trait ONLY.

```rust
impl IFileReaderPort for FileSystemSourceReader {
    fn read(&self, path: &FilePath) -> Result<FileContent, FileReadError> {
        // port implementation
    }
}
```

Do NOT put `Default`, `Clone`, `Debug`, `Display`, `From` impls in Block 2.

## Block 3 — Constructors, Std Traits, and Helpers

```rust
impl Default for FileSystemSourceReader {
    fn default() -> Self { Self }
}

impl FileSystemSourceReader {
    pub fn new() -> Self { Self }

    fn ensure_parent_dir(&self, path: &FilePath) -> Result<(), FileWriteError> {
        // private helper
    }
}
```

## Trait Placement Decision Rule

```text
Trait impl found in an infrastructure file?
  │
  ├─ Is it the domain port? (I<Name>Port)
  │   └─ YES → Block 2
  │
  └─ Is it a std/derive/utility trait? (Default, Clone, Debug, Display, From, etc.)
      └─ YES → Block 3
```
