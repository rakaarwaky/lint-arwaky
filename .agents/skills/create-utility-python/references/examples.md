# Utility Examples (BAD / GOOD — Python)

## Bad: Class in Utility

```python
# BAD — utility must not contain class definitions
class PathNormalizer:
    def __init__(self, separator: str):
        self.separator = separator

    def normalize(self, path: str) -> str:
        return path.replace("/", self.separator)
```

## Good: Stateless Module-Level Function

```python
# GOOD — utility contains only module-level functions
def normalize_path_separator(path: str, separator: str) -> str:
    return path.replace("/", separator)
```

## Bad: Business Logic in Utility

```python
# BAD — utility must not know about business rules
def is_valid_import(import_path: str) -> bool:
    # This knows about AES architecture — NOT domain-agnostic
    return import_path.startswith("shared.") or import_path.startswith("crate.common.")
```

## Good: Generic String Operation

```python
# GOOD — generic, domain-agnostic operation
def starts_with_module_prefix(path: str, prefix: str) -> bool:
    return path.startswith(prefix)
```

## Bad: Mutable State

```python
# BAD — utility must not have side effects or global state
_cache: dict[str, str] = {}

def cached_process(input_str: str) -> str:
    # BAD — global state mutation
    if input_str in _cache:
        return _cache[input_str]
    result = input_str.upper()
    _cache[input_str] = result
    return result
```

## Good: Pure Function

```python
# GOOD — pure function, deterministic output
def to_uppercase(input_str: str) -> str:
    return input_str.upper()
```

## Bad: Single-Module Dependency

```python
# BAD — if only one capability uses this, keep as private helper
def check_import_for_aes204(import_path: str) -> bool:
    # This knows about AES204 rule — domain-specific
    return "AES204" in import_path
```

## Good: Reusable Generic Function

```python
# GOOD — useful for any module that needs keyword checking
def contains_keyword(text: str, keyword: str) -> bool:
    return keyword in text
```

## I/O Example (Allowed)

```python
# OK — stateless, domain-agnostic, reusable across modules
from pathlib import Path
from typing import List


def read_file_content(path: Path) -> str:
    """Read file content — stateless, domain-agnostic, reusable."""
    return path.read_text()


def walk_directory(directory: Path, extensions: List[str]) -> List[str]:
    """Walk directory — stateless, domain-agnostic, reusable."""
    files = []
    for ext in extensions:
        files.extend(str(p) for p in directory.rglob(f"*{ext}"))
    return files
```
