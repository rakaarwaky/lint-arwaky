# Examples

## GOOD: Port Contract

```python
from abc import ABC, abstractmethod
from shared.file_system.taxonomy_file_content_vo import FileContent
from shared.file_system.taxonomy_file_path_vo import FilePath
from shared.file_system.taxonomy_file_read_error import FileReadError

class IFileSystemPort(ABC):
    @abstractmethod
    def read_file(self, path: FilePath) -> Result[FileContent, FileReadError]: ...
```

## GOOD: Protocol Contract

```python
from abc import ABC, abstractmethod
from shared.code_analysis.taxonomy_lint_result_vo import LintResult
from shared.code_analysis.taxonomy_source_vo import SourceContentVO

class IImportForbiddenProtocol(ABC):
    @abstractmethod
    def check(self, source: SourceContentVO) -> list[LintResult]: ...
```

## GOOD: Aggregate Contract

```python
from abc import ABC, abstractmethod
from shared.code_analysis.taxonomy_lint_result_vo import LintResult
from shared.import_rules.taxonomy_import_scan_request_vo import ImportScanRequest

class IImportRunnerAggregate(ABC):
    @abstractmethod
    def run(self, request: ImportScanRequest) -> list[LintResult]: ...
```

## BAD: Contract Contains Implementation

```python
class IFileSystemPort(ABC):
    @abstractmethod
    def read_file(self, path: FilePath) -> FileContent: ...

class FileAdapter(IFileSystemPort):
    def read_file(self, path: FilePath) -> FileContent:
        with open(path.value()) as f:  # BAD: implementation in contract
            return FileContent(f.read())
```

## BAD: Raw Primitives for Domain Values

```python
class IFileReaderPort(ABC):
    @abstractmethod
    def read(self, path: str) -> str: ...
```
