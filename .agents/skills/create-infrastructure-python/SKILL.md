---
name: create-infrastructure-python
description: "Create and validate infrastructure layer files following AES rules: 3-block structure, one class per file, port contracts, zero business logic."
version: 1.1.0
category: refactoring
tags:
  [
    python,
    aes,
    infrastructure,
    port,
    structure,
    aes404,
    3-block-structure,
    di,
    utility-extraction,
  ]
triggers:
  - "create infrastructure python"
  - "add infrastructure python"
  - "fix infrastructure structure python"
  - "create port python"
  - "infrastructure missing port python"
  - "verify infrastructure python"
  - "extract utility python"
  - "free function python"
dependencies: []
related:
  - create-capabilities-python
  - create-agent-python
  - enforce-1-class-per-file-python
  - trait-consolidation-python
  - module_logic_validator-python
  - fix-capability-structure-python
  - create-missing-protocols-python
---

# create-infrastructure-python

## Purpose

Create and validate Python **infrastructure layer** files following clean architecture rules. Ensures infrastructure contains zero business logic, inherits port ABCs, follows the 3-Block Structure, use DI for all fields, and extract stateless free functions to taxonomy utility modules.

## Rules

### Layer Boundaries (AES)

**Infrastructure Layer (`infrastructure_*.py`)**

| Allowed                                      | Forbidden                                        |
| -------------------------------------------- | ------------------------------------------------ |
| File I/O (`open()`, `Path()`)                | Business rules                                   |
| Network calls (`requests.`, `httpx.`)        | Domain logic                                     |
| Database operations (`sqlite3.`, `asyncpg.`) | Calculations (should be in capabilities)         |
| External API calls                           | Direct import from `agent_*`                     |
| Protocol/ABC implementation                  | Direct import from `capabilities_*`              |

### Structural Rules (All Layers)

- **1 file = 1 class** — each infrastructure file contains exactly ONE main class
- **All data classes in shared** — no dataclasses/Enums may be defined outside shared/taxonomy
- **Fields must use DI** — class fields should receive port interfaces via constructor
- **Helper methods stay in layer** — helper methods that support the class remain in the file
- **Utility functions → extract to taxonomy** — truly stateless, domain-agnostic functions (no `self`, no `cls`) MUST be extracted to `*_utility.py` modules in shared/taxonomy
- **No module-level `def` in infrastructure files** — free functions outside the class are forbidden; extract to `*_utility.py`

### Helper vs Utility Decision (The Litmus Test)

> **The Litmus Test:** "If I copy-paste this function to a completely different file, would it still work 100% the same without changing a single line of code?"
> - If **YES** → **Extract to Utility File**.
> - If **NO** (needs `self`, `cls`, or class context) → **Keep as Private Helper**.

#### When to Extract to Utility (`*_utility.py`)

Extract if **ALL** conditions are met:

1. **Stateless**: No `self`, no `cls`, no class-level state access
2. **Pure Function**: Input A always produces output B. No side effects (no I/O, no random, no global state mutation)
3. **Domain-Agnostic / Reusable**: Logic is general enough that other classes could use it in the future

#### When to Keep as Private Helper (Block 3)

Keep if **ANY** condition is met:

1. **Needs Instance State**: Accesses `self.field`
2. **Needs Class State**: Accesses class variables or `cls` attributes
3. **Tightly Coupled**: Logic is specific to this class only and doesn't make sense elsewhere (e.g., formatting error messages that reference this class name, mapping internal data to a class-specific output format)
4. **Factory Method**: `create_default()`, `from_config()`, `from_dict()` — specific to instantiating this class

#### I/O Blocker (CRITICAL)

A function can be stateless but STILL **cannot** be extracted to taxonomy if it has I/O:

- `open()`, `read()`, `write()`, `pathlib.Path.read_text()`
- `requests`, `urllib`, `httpx` (network)
- `sqlite3`, `psycopg2`, `sqlalchemy` (database)

**Rule:** Stateless + I/O = Keep in layer (or move to infrastructure), **NOT** taxonomy utility.

```python
def read_file_content(path: str) -> str:
    # Stateless ✓ (no self, no cls)
    # But uses open() ✗ (I/O)
    # → CANNOT extract to taxonomy utility
    # → Keep in infrastructure layer (this is correct — infra IS for I/O)
```

### The 3-Block Structure

Every implementation file MUST follow this exact order **within the class body**:

1. **Block 1 — Class Definition & Constructor**
   - `class <Type>(I<Name>Port):` declaration
   - `__init__` with DI fields (port interfaces)

2. **Block 2 — Port Methods** (Public Contract)
   - Methods that implement the port ABC's `@abstractmethod` signatures.
   - Contains **ONLY** the domain port methods.
   - **NO** dunder methods (`__repr__`, `__str__`, `__eq__`, `__hash__`, etc.) here.
   - **NO** factory classmethods (`create_default`, `from_config`) here.
   - **NO** `@staticmethod` helpers here.

3. **Block 3 — Dunder Methods, Factories & Helpers**
   - Dunder/utility methods: `__repr__`, `__str__`, `__eq__`, `__hash__`, `__copy__`, etc.
   - Factory classmethods: `create_default()`, `from_config()`, `from_dict()`
   - `@staticmethod` and `@classmethod` helpers **that depend on class semantics**
   - Private helper methods (`_helper_name`) that use `self`

**CRITICAL:** Block 2 is **RESERVED** for domain port methods ONLY. Dunder methods (`__repr__`, `__str__`, `__eq__`, `__hash__`) and factory classmethods belong in **Block 3** because they are utilities/constructors, not the public domain contract.

**CRITICAL:** Stateless free functions (no `self`, no `cls`, no class-level state) MUST be extracted OUT of the class into their own `*_utility.py` modules in shared/taxonomy. They do NOT belong in Block 3, Block 2, or at module level in infrastructure files.

#### Method Placement Decision Rule

```
Method / function found in an infrastructure file?
  │
  ├─ Module-level def (outside class)?
  │   └─ YES → EXTRACT to *_utility.py (ALWAYS forbidden in infrastructure)
  │
  ├─ Is it defined as @abstractmethod in the port ABC?
  │   └─ YES → Block 2 (Port Methods)
  │
  ├─ Is it a dunder method? (__repr__, __str__, __eq__, __hash__, __copy__)
  │   └─ YES → Block 3 (Dunder Methods & Helpers)
  │
  ├─ Is it a factory classmethod? (create_default, from_config, from_dict)
  │   └─ YES → Block 3 (Dunder Methods & Helpers)
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
      └─ YES → Block 3 (Private Helpers)
```

#### Example: Correct 3-Block Order

```python
from shared.common.taxonomy_path import FilePath
from shared.common.contract_file_reader_port import IFileReaderPort


# ─── Block 1: Class Definition & Constructor ──────────────
class FileCacheAdapter(IFileReaderPort):
    """Reads files from the local filesystem cache."""

    def __init__(self, cache_dir: FilePath) -> None:
        self._cache_dir = cache_dir


    # ─── Block 2: Port Methods (domain contract ONLY) ─────
    def read(self, path: FilePath) -> str:
        full_path = self._cache_dir.value / path.value
        with open(full_path, "r") as f:
            return f.read()


    # ─── Block 3: Dunder Methods, Factories & Helpers ─────
    def __repr__(self) -> str:
        return f"FileCacheAdapter(cache_dir={self._cache_dir!r})"

    def __eq__(self, other: object) -> bool:
        return isinstance(other, FileCacheAdapter) and self._cache_dir == other._cache_dir

    @classmethod
    def create_default(cls) -> "FileCacheAdapter":
        return cls(cache_dir=FilePath(".cache"))
```

#### Example: Extracted Utility Module

```python
# shared/common/taxonomy_file_utility.py
"""Stateless utility functions for file operations."""

import os
from pathlib import Path


def ensure_parent_dir(path: str) -> None:
    """Create parent directories if they don't exist."""
    Path(path).parent.mkdir(parents=True, exist_ok=True)


def normalize_path(path: str) -> str:
    """Normalize a file path for comparison."""
    return os.path.normpath(path)
```

### Port Rules

- **Every infrastructure class MUST inherit from a port ABC**
- **Port MUST define abstract methods for all public methods**
- **Port contains ONLY public/contract methods** — no helper methods
- **Helper methods stay in Block 3** (`@staticmethod`, private `_methods`)
- **Constructors in Block 1** — `__init__` receives port interfaces
- **Dunder methods (`__repr__`, `__str__`, `__eq__`, etc.) in Block 3**
- **Factory classmethods (`create_default`, `from_config`) in Block 3**
- **Stateless `@staticmethod` (no class dependency) → extract to `*_utility.py`**

## The Fundamental Question

> **"Is this file pure I/O or external system integration?"**

If yes → **`infrastructure_*.py` + inherit port ABC**
If no (has business logic) → **split into capabilities layer instead**

> **"Does this function need the class?"**

If no (`self`/`cls` unused) → **extract to `*_utility.py` in shared/taxonomy**
If yes → **keep in Block 3**

## Naming Convention

| Layer                    | File Pattern            | Protocol File                    | Protocol Name        |
| ------------------------ | ----------------------- | -------------------------------- | -------------------- |
| **Capabilities**   | `capabilities_*.py`   | `contract_<name>_protocol.py`  | `I<Name>Protocol`  |
| **Infrastructure** | `infrastructure_*.py` | `contract_<name>_port.py`      | `I<Name>Port`      |
| **Agents**         | `agent_*.py`          | `contract_<name>_aggregate.py` | `I<Name>Aggregate` |
| **Utility**        | `taxonomy_<name>_utility.py` | —                         | — (free functions) |

## Detection Patterns

### BAD: Infrastructure Without Port (AES404)

```python
# BAD: No port inheritance
class FileCache:
    def read(self): ...
```

### BAD: Business Logic in Infrastructure

```python
# BAD: Business logic in infrastructure layer
class OrphanFileCache:
    def analyze(self, content: str) -> bool:
        # ← DOMAIN LOGIC — should be in capabilities
        is_orphan = "orphan" in content
        return is_orphan
```

### BAD: Dataclass in Layer File

```python
# BAD: Domain data defined in infrastructure layer
@dataclass
class CacheEntry:  # ← DATA CLASS — should be in shared/taxonomy
    key: str
    value: str
    timestamp: int

class OrphanFileCache:
    entry: CacheEntry  # ← concrete type, not DI
```

### BAD: Dunder Methods in Block 2

```python
# BAD: __repr__ / __eq__ mixed in with port methods
class FileCacheAdapter(IFileReaderPort):
    def __init__(self, cache_dir: FilePath): ...

    def __repr__(self) -> str:           # ← Block 2 position, NOT a port method
        return "FileCacheAdapter()"

    def read(self, path: FilePath) -> str:  # ← pushed down
        ...

    def __eq__(self, other) -> bool:     # ← also in Block 2 position
        return isinstance(other, FileCacheAdapter)
```

### BAD: Module-Level Free Function in Infrastructure File

```python
# BAD: Free function outside class in infrastructure file
# infrastructure_file_adapter.py

def ensure_parent_dir(path: str) -> None:   # ← FREE FUNCTION — extract to utility
    Path(path).parent.mkdir(parents=True, exist_ok=True)

def normalize_path(path: str) -> str:       # ← FREE FUNCTION — extract to utility
    return os.path.normpath(path)

class FileCacheAdapter(IFileReaderPort):
    def read(self, path: FilePath) -> str:
        normalized = normalize_path(path.value)
        ...
```

### BAD: Stateless @staticmethod That Should Be Extracted

```python
# BAD: @staticmethod with zero class dependency — belongs in utility
class FileCacheAdapter(IFileReaderPort):

    @staticmethod
    def normalize_path(path: str) -> str:   # ← no self, no cls, pure logic
        return os.path.normpath(path)

    @staticmethod
    def ensure_parent_dir(path: str) -> None:  # ← no self, no cls, pure logic
        Path(path).parent.mkdir(parents=True, exist_ok=True)

    def read(self, path: FilePath) -> str:
        self.ensure_parent_dir(path.value)  # ← could be a free function
        ...
```

### GOOD: Class with Shared Data

```python
# GOOD: All data from shared, fields use ports
from shared.common.taxonomy_path import FilePath
from contract_file_reader_port import IFileReaderPort

class FileCacheAdapter:
    def __init__(self, cache_dir: FilePath):
        self._cache_dir = cache_dir  # ← DI via port
```

### GOOD: Correct 3-Block with Dunder Methods

```python
# GOOD: Port methods in Block 2, dunders + factories in Block 3
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

### GOOD: Extracted to Taxonomy Utility

```python
# GOOD: shared/common/taxonomy_file_utility.py

import os
from pathlib import Path

def ensure_parent_dir(path: str) -> None:
    """Create parent directories if they don't exist."""
    Path(path).parent.mkdir(parents=True, exist_ok=True)

def normalize_path(path: str) -> str:
    """Normalize a file path for comparison."""
    return os.path.normpath(path)
```

```python
# GOOD: infrastructure_file_adapter.py (consumer)

from shared.common.taxonomy_file_utility import ensure_parent_dir, normalize_path

class FileCacheAdapter(IFileReaderPort):

    def read(self, path: FilePath) -> str:
        ensure_parent_dir(path.value)        # ← imported from utility
        normalized = normalize_path(path.value)  # ← imported from utility
        ...
```

## Workflow

### Step 1: Analyze File

Read file and check for mixed responsibilities. Ask: **"Is this code in the right layer?"**

- If it has business logic → **MOVE to Capabilities** (AES404)
- If pure I/O/external integration → continue to Step 2

### Step 2: Check for Missing Port

Does the infrastructure class inherit from a port ABC? If no → create one.

```bash
# Find infrastructure without port inheritance
grep -rn "^class " modules/*/src/infrastructure_*.py | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    class=$(echo "$line" | grep -oP 'class \K[a-zA-Z_]+')
    grep -q "Port" "$file" || echo "MISSING: $file has $class without port"
done
```

### Step 3: Create Port File (if missing)

Create `contract_<name>_port.py` in the shared module with abstract methods.

**Port location:**

| Module     | Port Path                                            |
| ---------- | ---------------------------------------------------- |
| compositor | `modules/shared/src/compositor/contract_*_port.py` |
| animator   | `modules/shared/src/animator/contract_*_port.py`   |
| scripting  | `modules/shared/src/scripting/contract_*_port.py`  |

### Step 4: Enforce 3-Block Structure

Reorganize into strict 3-block order within the class body:

1. `class <Type>(I<Name>Port):` + `__init__` (class definition with DI fields)
2. Port `@abstractmethod` implementations (**domain port methods ONLY**)
3. Dunder methods (`__repr__`, `__str__`, `__eq__`), factory classmethods (`create_default`, `from_config`), `@staticmethod`/`@classmethod` helpers that use class state, and private `_helpers`

### Step 5: Verify Class Discipline

- **1 file = 1 class** — no multiple classes in one file
- **All dataclasses in shared/taxonomy** — domain types must be imported, not defined locally
- **Fields use ports** — constructor receives port interfaces, not concrete types
- **No standalone functions (no `self`) remain in Block 3** — extract to `*_utility.py` modules

### Step 6: Extract Free Functions to Utility

Scan the file for functions that have **no `self`/`cls` dependency**:

```bash
# Find module-level functions (outside class) — ALWAYS forbidden
grep -n "^def " modules/*/src/infrastructure_*.py

# Find @staticmethod inside class (no self, no cls)
grep -B1 -A5 "@staticmethod" modules/*/src/infrastructure_*.py

# Find @classmethod that doesn't use cls
grep -B1 -A5 "@classmethod" modules/*/src/infrastructure_*.py
```

For each candidate, ask:

| Question | YES → | NO → |
|----------|-------|------|
| Uses `self` or instance state? | Keep in Block 3 | Continue ↓ |
| Uses `cls` or class-level state? | Keep as `@classmethod` in Block 3 | Continue ↓ |
| Tightly coupled to class semantics (e.g., references class constants)? | Keep as `@staticmethod` in Block 3 | Continue ↓ |
| Pure logic, deterministic, no side effects? | **Extract to `*_utility.py`** | Keep in Block 3 |
| Domain-agnostic (not specific to this class)? | **Extract to `*_utility.py`** | Keep in Block 3 |

**Extraction process:**

1. Create `modules/shared/src/<domain>/taxonomy_<name>_utility.py`
2. Move function(s) to utility file with docstrings
3. Extract magic constants to `taxonomy_<name>_constant.py` if needed
4. Add import in infrastructure file: `from shared.<domain>.taxonomy_<name>_utility import func_name`
5. Remove original function from infrastructure file
6. Register utility module in `__init__.py` if needed
7. Verify: `python -c "from shared.<domain>.taxonomy_<name>_utility import *"`

### Step 7: Verify Layer Compliance

Check forbidden imports and business logic patterns:

```bash
# Check for business logic in infrastructure
grep -n "is_orphan\|analyze\|validate" modules/*/src/infrastructure_*.py

# Check for forbidden imports
grep -n "capabilities_\|agent_" modules/*/src/infrastructure_*.py
```

### Step 8: Verify

Run syntax check to confirm no violations.

```bash
python -c "import <module>"
```

## Import Strategy

When deciding where a function belongs:

### Option A: Extract to Taxonomy Utility (Standalone Free Functions)

Use when the code is **stateless, pure logic** with no side effects:

| Condition                                     | Example                                       |
| --------------------------------------------- | --------------------------------------------- |
| No `self`, no `cls`, no class state           | `normalize_path(path: str) -> str`            |
| All data via parameters                       | `ensure_parent_dir(path: str) -> None`        |
| Deterministic, no side effects                | `is_valid_extension(name: str) -> bool`       |

```python
# taxonomy_file_utility.py (SHARED / TAXONOMY)
def normalize_path(path: str) -> str:
    return os.path.normpath(path)

# infrastructure_file_adapter.py (CONSUMER)
from shared.common.taxonomy_file_utility import normalize_path
```

### Option B: Keep as Instance/Class Method (Stateful or Side-Effectful)

Use when the code requires **instance state, class state, or side effects**:

| Condition                     | Example                                         |
| ----------------------------- | ----------------------------------------------- |
| Uses `self` / instance fields | `self._cache.get(key)`                          |
| Uses `cls` / class-level config | `cls._registry[name]`                         |
| Has side effects / I/O        | File operations, logging with context           |
| Tightly coupled to class semantics | References class-level constants or types  |

```python
# infrastructure_file_adapter.py (STAYS IN CLASS — Block 3)
class FileCacheAdapter(IFileReaderPort):
    _DEFAULT_CACHE_DIR = ".cache"  # class-level constant

    def __init__(self, cache_dir: FilePath):
        self._cache_dir = cache_dir

    def _resolve_path(self, path: str) -> Path:  # uses self → stays
        return self._cache_dir.value / path

    @classmethod
    def from_env(cls) -> "FileCacheAdapter":  # uses cls → stays
        return cls(cache_dir=FilePath(os.environ.get("CACHE_DIR", cls._DEFAULT_CACHE_DIR)))
```

### Decision Tree

```
Function found in infrastructure file?
  │
  ├─ Module-level def (outside class)?
  │   └─ YES → EXTRACT to *_utility.py (ALWAYS forbidden in infrastructure)
  │
  ├─ @staticmethod inside class?
  │   ├─ Pure logic, no class dependency?
  │   │   └─ YES → EXTRACT to *_utility.py
  │   └─ Tightly coupled to class semantics?
  │       └─ YES → Keep as @staticmethod in Block 3
  │
  ├─ @classmethod?
  │   ├─ Factory (create_default, from_config)?
  │   │   └─ YES → Keep in Block 3
  │   ├─ Uses cls or class-level state?
  │   │   └─ YES → Keep in Block 3
  │   └─ Pure logic, no cls dependency?
  │       └─ YES → Convert to free fn → EXTRACT to *_utility.py
  │
  ├─ Instance method (self)?
  │   ├─ Defined in port ABC?
  │   │   └─ YES → Block 2
  │   └─ Private helper?
  │       └─ YES → Block 3
  │
  └─ Dunder method (__repr__, __eq__, etc.)?
      └─ YES → Block 3
```

## Verification Checklist

- [ ] File follows the **3-Block Structure** (Class + `__init__` → Port Methods → Dunders/Factories/Helpers).
- [ ] **Block 2 contains ONLY port `@abstractmethod` implementations**. No dunder methods, no `@staticmethod`, no factory classmethods in Block 2.
- [ ] **Dunder methods** (`__repr__`, `__str__`, `__eq__`, `__hash__`) and **factory classmethods** (`create_default`, `from_config`) are in **Block 3**.
- [ ] Infrastructure class inherits a port ABC.
- [ ] Port contains **only** public/contract methods (no helper methods).
- [ ] Helper methods are in Block 3.
- [ ] Constructors receive port interfaces via `__init__`.
- [ ] **No module-level `def` functions** exist outside the class in infrastructure files.
- [ ] **No stateless `@staticmethod`** (zero class dependency) remains in class — extracted to `*_utility.py`.
- [ ] Stateless utilities exist in their own `*_utility.py` files in shared/taxonomy.
- [ ] **1 file = 1 class** — no multiple classes in one file.
- [ ] All dataclasses imported from shared/taxonomy (none defined locally).
- [ ] Constructor fields use port interfaces, not concrete types.
- [ ] **Zero business logic** in infrastructure layer (no domain rules, no calculations).
- [ ] No forbidden imports (no capabilities\_\_, no agent\_\_).
- [ ] Port module is registered in the shared module's `__init__.py`.
- [ ] Utility module is registered in the shared module's `__init__.py`.
- [ ] `python -c "import <module>"` passes without errors.

## Quick Commands

```bash
# Verify 3-Block Structure order (rough check)
grep -n "^class\|^    def \|^    @" modules/<module>/src/infrastructure_*.py

# Find infrastructure without port inheritance
grep -rn "^class " modules/*/src/infrastructure_*.py | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    class=$(echo "$line" | grep -oP 'class \K[a-zA-Z_]+')
    grep -q "Port" "$file" || echo "MISSING: $file has $class without port"
done

# Ensure port does NOT contain helper methods
grep -E "def (helper|util|private|_)" modules/shared/src/contract_*_port.py || echo "Clean: No helpers in port"

# Check for business logic in infrastructure
grep -n "is_orphan\|analyze\|validate\|business" modules/*/src/infrastructure_*.py

# Check for dataclasses defined in layer files
grep -rn "^@dataclass\|^class.*Enum" modules/*/src/ | grep -v "shared/" | grep infrastructure

# Check for concrete type fields (non-port)
grep -n "__init__" modules/*/src/infrastructure_*.py | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    grep -A5 "def __init__" "$file" | grep -v "Protocol\|Port\|Aggregate" || echo "NON-PORT FIELD: $file"
done

# Find module-level free functions in infrastructure files (ALWAYS forbidden)
grep -n "^def [a-z_]*(" modules/*/src/infrastructure_*.py

# Find @staticmethod that may need extraction (no self, no cls)
grep -B1 -A10 "@staticmethod" modules/*/src/infrastructure_*.py

# Detect dunder methods appearing BEFORE port methods (wrong block order)
python3 -c "
import re, sys
for f in sys.argv[1:]:
    lines = open(f).readlines()
    first_dunder = first_port = None
    for i, l in enumerate(lines):
        m = re.match(r'\s+def (__\w+__)\(', l)
        if m and m.group(1) not in ('__init__', '__init_subclass__') and first_dunder is None:
            first_dunder = i + 1
        m2 = re.match(r'\s+def ([a-z]\w+)\(', l)
        if m2 and not m2.group(1).startswith('_') and first_port is None:
            first_port = i + 1
    if first_dunder and first_port and first_dunder < first_port:
        print(f'VIOLATION: {f} — dunder (line {first_dunder}) before port method (line {first_port})')
" modules/*/src/infrastructure_*.py

# Find standalone functions in class files (should be extracted to utility)
grep -n "^def [a-z_]*(\s*[^self])" modules/*/src/infrastructure_*.py

# Check syntax
python -c "import <module>"
```

## Common Mistakes (AVOID)

- ❌ **Putting business logic in infrastructure**: Domain rules, calculations, and validation MUST be in capabilities layer.
- ❌ **Defining dataclasses in layer files**: Domain data must be in shared/taxonomy. Only the class belongs in layer files.
- ❌ **Using concrete types as constructor fields**: Constructor should receive port interfaces, not concrete implementations.
- ❌ **Putting helper methods in the port**: This violates encapsulation and forces all implementors to write boilerplate.
- ❌ **Mixing Block 2 and Block 3**: Do not interleave port methods and helper methods. Keep them in separate sections.
- ❌ **Placing utilities in class body**: Stateless functions (no `self`) MUST be extracted to standalone `*_utility.py` modules.
- ❌ **Creating "God Ports"**: If a port has >10 methods or mixes unrelated concerns, split it into multiple ports.
- ❌ **Multiple classes in one file**: Each file should have exactly ONE class. Use `consolidate-files-python` if merging multiple files.
- ❌ **Placing dunder methods (`__repr__`, `__str__`, `__eq__`) in Block 2**: Block 2 is RESERVED for port method implementations ONLY. Dunders are utilities and belong in Block 3.
- ❌ **Placing factory classmethods (`create_default`, `from_config`) before port methods**: Factories are constructors and belong in Block 3, after port methods.
- ❌ **Mixing `__init__` into Block 2**: `__init__` is part of Block 1 (class definition & constructor), not a port method.
- ❌ **Leaving module-level `def` in infrastructure files**: Free functions outside the class MUST be extracted to `*_utility.py` in shared/taxonomy. No exceptions.
- ❌ **Keeping stateless `@staticmethod` in class**: If a `@staticmethod` uses no `self`, no `cls`, and no class-level state, it belongs in `*_utility.py`, not in the class body.
