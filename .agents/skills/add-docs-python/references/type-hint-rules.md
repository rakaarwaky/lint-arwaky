# Type Hint Rules

## Purpose

Type hints provide type safety and IDE support.

## Rules

1. **All function parameters** — must have type hints
2. **All function return types** — must have type hints
3. **Use `typing` module** — for complex types (`List`, `Dict`, `Optional`, `Union`)
4. **Use `from __future__ import annotations`** — for forward references
5. **Use `Optional[T]`** — for nullable types (equivalent to `T | None`)
6. **Use `Union[T1, T2]`** — for multiple types (equivalent to `T1 | T2`)
7. **Use `Callable`** — for function types

## Template

```python
from __future__ import annotations
from typing import Any, Optional, Union


# Function with type hints
def validate(data: dict[str, Any]) -> tuple[bool, str]:
    """Validate data against the import rule."""
    # ...


# Optional parameter
def find(name: str, optional: Optional[str] = None) -> list[str]:
    """Find items by name."""
    # ...


# Union type
def process(value: Union[str, int]) -> str:
    """Process value."""
    # ...
```

## Anti-Patterns

- ❌ Missing type hints → add types to all parameters and returns
- ❌ Using `Any` without reason → use specific type if known
- ❌ Using `type: ignore` → fix root cause instead
- ❌ Missing `from __future__ import annotations` → add for forward references
- ❌ Missing `typing` imports → import needed types
