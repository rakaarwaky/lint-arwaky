# Error Handling Rules

## Rule 1: Do not silently discard errors

Forbidden:

```python
content = open(path.value()).read() or ""
```

## Rule 2: Fallible port methods should return `Result` or raise

```python
def read(self, path: FilePath) -> Result[FileContent, FileReadError]: ...
```

## Rule 3: Use descriptive error types

```python
class FileReadError(Enum):
    IO = "io"
    VALIDATION = "validation"
```

## Rule 4: Infrastructure should not produce lint results directly

Infrastructure should return data, errors, or VOs. Lint violations belong to capabilities.

Bad:

```python
def read(self, path: FilePath) -> list[LintResult]: # BAD
```

Good:

```python
def read(self, path: FilePath) -> Result[FileContent, FileReadError]: # OK
```
