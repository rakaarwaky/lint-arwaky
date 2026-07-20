# Examples

## GOOD: Port Contract

```rust
use async_trait::async_trait;
use shared::<name-feature>::taxonomy_file_content_vo::FileContent;
use shared::<name-feature>::taxonomy_file_path_vo::FilePath;
use shared::<name-feature>::taxonomy_file_read_error::FileReadError;

#[async_trait]
pub trait IFileSystemProtocol: Send + Sync {
    async fn read_file(&self, path: &FilePath) -> Result<FileContent, FileReadError>;
}
```

## GOOD: Protocol Contract

```rust
use shared::<name-feature>::taxonomy_result_vo::<ResultVO>;
use shared::<name-feature>::taxonomy_source_vo::SourceContentVO;

pub trait IImportForbiddenProtocol: Send + Sync {
    fn check(&self, source: &SourceContentVO) -> Vec<<ResultVO>>;
}
```

## GOOD: Aggregate Contract

```rust
use shared::<name-feature>::taxonomy_result_vo::<ResultVO>;
use shared::<name-feature>::taxonomy_scan_request_vo::<ScanRequest>VO;

pub trait IImportRunnerAggregate: Send + Sync {
    fn run(&self, request: &<ScanRequest>VO) -> Vec<<ResultVO>>;
}
```

## BAD: Contract Contains Implementation

```rust
pub trait IFileSystemProtocol: Send + Sync {
    async fn read_file(&self, path: &FilePath) -> Result<FileContent, FileReadError>;
}

impl IFileSystemProtocol for FileAdapter {
    async fn read_file(&self, path: &FilePath) -> Result<FileContent, FileReadError> {
        todo!() // BAD: implementation belongs in infrastructure
    }
}
```

## BAD: Raw Primitives for Domain Values

```rust
pub trait IFileReaderProtocol: Send + Sync {
    fn read(&self, path: &str) -> Result<String, std::io::Error>;
}
```
