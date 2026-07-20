# Examples

## GOOD: Port Contract

```python
from abc import ABC, abstractmethod
from shared.<name-feature>.taxonomy_file_content_vo import FileContent
from shared.<name-feature>.taxonomy_file_path_vo import FilePath
from shared.<name-feature>.taxonomy_file_read_error import FileReadError

class IFileSystemProtocol(ABC):
    @abstractmethod
    def read_file(self, path: FilePath) -> Result[FileContent, FileReadError]: ...
```

## GOOD: Protocol Contract

```python
from abc import ABC, abstractmethod
from shared.<name-feature>.taxonomy_result_vo import <ResultVO>
from shared.<name-feature>.taxonomy_source_vo import SourceContentVO

class IImportForbiddenProtocol(ABC):
    @abstractmethod
    def check(self, source: SourceContentVO) -> list[<ResultVO>]: ...
```

## GOOD: Aggregate Contract

```python
from abc import ABC, abstractmethod
from shared.<name-feature>.taxonomy_result_vo import <ResultVO>
from shared.<name-feature>.taxonomy_scan_request_vo import <ScanRequest>VO

class IImportRunnerAggregate(ABC):
    @abstractmethod
    def run(self, request: <ScanRequest>VO) -> list[<ResultVO>]: ...
```

## BAD: Contract Contains Implementation

```python
class IFileSystemProtocol(ABC):
    @abstractmethod
    def read_file(self, path: FilePath) -> FileContent: ...

class FileAdapter(IFileSystemProtocol):
    def read_file(self, path: FilePath) -> FileContent:
        with open(path.value()) as f:  # BAD: implementation in contract
            return FileContent(f.read())
```

## BAD: Raw Primitives for Domain Values

```python
class IFileReaderProtocol(ABC):
    @abstractmethod
    def read(self, path: str) -> str: ...
```
