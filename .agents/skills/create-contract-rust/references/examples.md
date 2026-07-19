# Examples

## GOOD: Port Contract

```rust
use async_trait::async_trait;
use crate::file_system::taxonomy_file_content_vo::FileContent;
use crate::file_system::taxonomy_file_path_vo::FilePath;
use crate::file_system::taxonomy_file_read_error::FileReadError;

#[async_trait]
pub trait IFileSystemPort: Send + Sync {
    async fn read_file(&self, path: &FilePath) -> Result<FileContent, FileReadError>;
}
```

## GOOD: Protocol Contract

```rust
use crate::code_analysis::taxonomy_lint_result_vo::LintResult;
use crate::code_analysis::taxonomy_source_vo::SourceContentVO;

pub trait IImportForbiddenProtocol: Send + Sync {
    fn check(&self, source: &SourceContentVO) -> Vec<LintResult>;
}
```

## GOOD: Aggregate Contract

```rust
use crate::code_analysis::taxonomy_lint_result_vo::LintResult;
use crate::import_rules::taxonomy_import_scan_request_vo::ImportScanRequest;

pub trait IImportRunnerAggregate: Send + Sync {
    fn run(&self, request: &ImportScanRequest) -> Vec<LintResult>;
}
```

## BAD: Contract Contains Implementation

```rust
pub trait IFileSystemPort: Send + Sync {
    async fn read_file(&self, path: &FilePath) -> Result<FileContent, FileReadError>;
}

impl IFileSystemPort for FileAdapter {
    async fn read_file(&self, path: &FilePath) -> Result<FileContent, FileReadError> {
        todo!() // BAD: implementation belongs in infrastructure
    }
}
```

## BAD: Raw Primitives for Domain Values

```rust
pub trait IFileReaderPort: Send + Sync {
    fn read(&self, path: &str) -> Result<String, std::io::Error>;
}
```
