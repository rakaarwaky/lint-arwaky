# Examples

## BAD: Infrastructure Without Port (AES404)

```python
class FileCache:
    def read(self) -> str:
        # public behavior without port ABC
        ...
```

Fix:

```python
class FileCache(IFileCachePort):
    def read(self) -> str:
        # contract implementation
        ...
```

## BAD: Business Logic in Infrastructure

```python
class OrphanFileCache:
    def analyze(self, content: FileContent) -> bool:
        # BAD: domain logic
        return "orphan" in content.value
```

Fix: Move analysis to capabilities.

## BAD: Dunder Methods in Block 2

```python
class FileCacheAdapter(IFileReaderPort):
    def __init__(self, cache_dir: FilePath): ...

    def __repr__(self) -> str:           # ← Block 2 position, NOT a port method
        return "FileCacheAdapter()"

    def read(self, path: FilePath) -> str:  # ← pushed down
        ...
```

Fix: Move `__repr__` to Block 3.

## GOOD: Correct 3-Block with Dunder Methods

```python
class FileCacheAdapter(IFileReaderPort):

    def __init__(self, cache_dir: FilePath) -> None:  # Block 1: constructor
        self._cache_dir = cache_dir

    def read(self, path: FilePath) -> str:  # Block 2: port method ONLY
        ...

    def __repr__(self) -> str:               # Block 3: dunder = utility
        return f"FileCacheAdapter(cache_dir={self._cache_dir!r})"

    @classmethod
    def create_default(cls) -> "FileCacheAdapter":  # Block 3: factory
        return cls(cache_dir=FilePath(".cache"))
```
