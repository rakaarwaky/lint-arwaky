# Primitive and VO Rules

Infrastructure public contracts should use shared VOs for domain data.

Bad:

```python
class IFileWriterPort(ABC):
    @abstractmethod
    def write(self, path: str, content: str) -> None: ...
```

Good:

```python
class IFileWriterPort(ABC):
    @abstractmethod
    def write(self, path: FilePath, content: FileContent) -> Result[None, FileWriteError]: ...
```

## Primitive Policy

| Primitive | Rule |
| --------- | ---- |
| `str`     | Forbidden for domain fields and public contract values. Use VO. |
| `int`     | Forbidden for domain values. Use VO. |
| `float`   | Forbidden for domain values. Use VO. |
| `bool`    | Allowed for technical toggles when no richer VO is needed. |

Prefer VOs for: file paths, URLs, timeouts, durations, cache keys, cache values, query results, identifiers, messages.
