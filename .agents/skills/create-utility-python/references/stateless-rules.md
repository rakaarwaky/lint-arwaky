# Stateless Rules (Utility — Python)

## Rule 1: No Classes

Utility files must NOT contain:

- `class` definitions
- `self` parameter in functions
- Class attributes or instance state

Bad:

```python
class UtilityHelper:
    def __init__(self):
        self.cache = {}

    def process(self, input_str: str) -> str:
        # BAD — has state
        pass
```

Good:

```python
def process(input_str: str) -> str:
    # pure function, no state
    return input_str.upper()
```

## Rule 2: Pure Functions Only

A utility function must satisfy:

- **Deterministic:** same input → same output every time
- **No side effects:** no I/O (unless domain-agnostic + reusable), no network, no database
- **No randomness:** no `random`, no `secrets`
- **No global state:** no module-level mutable variables

Bad:

```python
import time

def get_timestamp() -> float:
    return time.time()  # BAD — not deterministic
```

Good:

```python
def normalize_path(path: str) -> str:
    return path.replace("\\", "/").lstrip("./")
```

## Rule 3: Domain Agnostic

Utility functions must NOT know about:

- Architecture layer names (agent, capabilities, contract, etc.)
- Business domain rules (naming conventions, import policies)
- Specific capability logic (how a checker validates)

Bad:

```python
# BAD — knows about architecture layers
def is_in_agent_layer(filename: str) -> bool:
    return filename.startswith("agent_")
```

Good:

```python
# GOOD — generic string operation
def starts_with_prefix(prefix: str, filename: str) -> bool:
    return filename.startswith(prefix)
```

## Rule 4: Reusable Across Modules

Utility functions must be useful for multiple modules.

If a function is only used by one capability or one agent → keep as private helper in that module.

Decision Tree:

```text
Found reusable code?
  │
  ├─ Used by ≥2 modules?
  │   ├─ YES → check stateless + pure + domain-agnostic
  │   │         └─ All YES → extract to utility
  │   └─ NO → keep as private helper in the layer file
```

## I/O Exception

Utility CAN perform I/O if ALL conditions are met:

1. Stateless (no `self`, no class attributes)
2. Domain-agnostic (no business knowledge)
3. Reusable across multiple modules

Good examples:

```python
# OK — stateless, domain-agnostic, reusable
from pathlib import Path
from typing import List

def walk_source_files(directory: Path, extensions: List[str]) -> List[Path]:
    """Walk directory and return files matching extensions."""
    files = []
    # I/O is allowed here
    for ext in extensions:
        files.extend(directory.rglob(f"*{ext}"))
    return files
```
