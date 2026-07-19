# The 3-Block Structure

1. **Block 1 — Class Definition & Constructor**
2. **Block 2 — Port ABC Method Implementation**
3. **Block 3 — Dunder Methods, Factories, and Private Helpers**

## Block 1 — Class Definition & Constructor

```python
class FileSystemSourceReader(IFileReaderPort):
    def __init__(self) -> None:
        pass
```

## Block 2 — Public Contract

Block 2 is RESERVED for the domain port ABC methods ONLY.

```python
class FileSystemSourceReader(IFileReaderPort):
    def read(self, path: FilePath) -> Result[FileContent, FileReadError]:
        # port implementation
        ...
```

Do NOT put `__repr__`, `__str__`, `__eq__`, `@classmethod create_default()`, `@staticmethod helpers` in Block 2.

## Block 3 — Dunder Methods, Factories, and Helpers

```python
class FileSystemSourceReader(IFileReaderPort):
    def __repr__(self) -> str:
        return "FileSystemSourceReader()"

    @classmethod
    def create_default(cls) -> "FileSystemSourceReader":
        return cls()

    def _ensure_parent_dir(self, path: FilePath) -> Result[None, FileWriteError]:
        # private helper
        ...
```

## Method Placement Decision Rule

```text
Method / function found in an infrastructure file?
  │
  ├─ Module-level def (outside class)?
  │   └─ YES → EXTRACT to *_utility.py (ALWAYS forbidden in infrastructure)
  │
  ├─ Is it defined as @abstractmethod in the port ABC?
  │   └─ YES → Block 2
  │
  ├─ Is it a dunder method? (__repr__, __str__, __eq__, __hash__, __copy__)
  │   └─ YES → Block 3
  │
  ├─ Is it a factory classmethod? (create_default, from_config, from_dict)
  │   └─ YES → Block 3
  │
  ├─ Is it @staticmethod / @classmethod?
  │   ├─ Uses cls or class-level state?
  │   │   └─ YES → Block 3 (keep as @classmethod)
  │   ├─ Tightly coupled to class semantics?
  │   │   └─ YES → Block 3 (keep as @staticmethod)
  │   └─ Pure logic, no class dependency?
  │       └─ YES → EXTRACT to *_utility.py
  │
  └─ Is it a private helper using self?
      └─ YES → Block 3
```
